use std::collections::{HashMap, VecDeque};
use crate::op::{Op, Modifiers};
use crate::parsed::Parsed;
use crate::stack::Stack;
use crate::stack_error::StackError;
use crate::types::{Params, Constraint, Type};

pub struct Binding {
    pub function: bool,
    pub constant: bool,
    pub value: Parsed
}

pub fn run(stack: &mut Stack<Parsed>, input: &mut VecDeque<Parsed>, bindings: &mut HashMap<String, Binding>) {
    while !input.is_empty() {
        if let Some(p) = input.pop_front() {
            match p.clone() {
                Parsed::Error(e) => {
                    println!("runtime error: {}", e);
                    stack.push(p);
                    break;
                }
                Parsed::Symbol(s) => {
                    if let Some (val) = bindings.get(&s) {
                        if val.function {
                            run(stack, &mut VecDeque::from(val.value.get_contents().unwrap()), bindings)
                        } else {
                            stack.push(val.value.clone())
                        }
                    } else {
                        stack.push(p.clone())
                    }
                },
                Parsed::List(s) => {
                    stack.push(Parsed::List(s.iter().map(|p|
                        match p {
                        Parsed::Symbol(s) => {
                            if let Some (val) = bindings.get(s.as_str()) {
                                val.value.clone()
                            } else {
                                p.clone()
                            }
                        },
                        _ => p.clone()
                    }).collect()));
                }
                Parsed::Function(op) => {
                    exec_op(op, stack, input, bindings )
                },
                other => {
                    stack.push(other)
                }
            }
        }
    }
}



fn exec_op(op: Op, stack: &mut Stack<Parsed>, input: &mut VecDeque<Parsed>, bindings: &mut HashMap<String, Binding>) {
    let signature = op.get_signature();
    match &signature.stack_args {
        Params::Nullary => {
            match get_closures(signature.modifiers, input) {
                Ok (closures) => {
                    let ret = op.exec_nullary(closures, bindings);
                    if signature.ret.is_satisfied_by(&ret.get_type()) &&
                        signature.ret != Constraint::Void {
                        stack.push(ret)
                    }
                },
                Err(e) => {
                    stack.push(Parsed::Error(e));
                }
            }
        }
        Params::Unary(c) => {
            match get_closures(signature.modifiers, input) {
                Ok(closures) => {
                    let arg = stack.pop().unwrap();
                    if c.is_satisfied_by(&arg.get_type()) {
                        let res = op.exec_unary(arg, closures);
                        //TODO: Error handling
                        if signature.ret.is_satisfied_by(&res.get_type()) {
                            match res {
                                Parsed::Quotation(q) => {
                                    run(stack, &mut q.clone(), bindings)
                                },
                                Parsed::Void => {},
                                _ => stack.push(res)
                            }
                        } else {
                            println!("{} {} {}", res, res.get_type(), signature.ret);
                        };
                    } else {
                        print_mismatch_arg(op, signature.stack_args, Args::Unary(arg))
                    }
                },
                Err(e) => {
                    stack.push(Parsed::Error(e));
                }
            }
        },
        Params::Binary(c1, c2) => {
            let rhs = stack.pop().unwrap();
            let lhs = stack.pop().unwrap();
            // Checks that the constraints of the function signature is satisfied.
            if c1.is_satisfied_by(&lhs.get_type()) &&
                c2.is_satisfied_by(&rhs.get_type()) {
                if let Ok(closures) = get_closures(signature.modifiers, input) {
                    let res = op.exec_binary(&lhs, &rhs, closures, bindings);
                    if signature.ret.is_satisfied_by(&res.get_type()) {
                        match res {
                            Parsed::Quotation(q) => {
                                run(stack, &mut q.clone(), bindings)
                            },
                            Parsed::Void => {},
                            _ => stack.push(res)
                        }
                    } else {
                        println!("{}", res);
                    };
                }

            } else {
                print_mismatch_arg(op, signature.stack_args, Args::Binary(
                    lhs,
                    rhs)
                )
            }
        },
        _ => {}
    }
}

fn print_mismatch_arg(op: Op, exp: Params, got: Args) {
    match (exp, got) {
        (Params::Unary(expected), Args::Unary(actual)) => {
            println!("err: argument of type \x1b[33m{}\x1b[0m with value \x1b[33m{}\x1b[0m does not satisfy constraint in \
            the function \x1b[36m{}\x1b[0m, with signature (\x1b[31m{}\x1b[0m -> {}).", actual.get_type(), actual, op, expected, op.get_signature().ret)
        },
        (Params::Binary(exp1, exp2), Args::Binary(act1, act2)) => {
            println!("bug: {} {}", act1, act2);
            let lhs = !exp1.is_satisfied_by(&act1.get_type());
            let rhs = !exp2.is_satisfied_by(&act2.get_type());
            let mut do_grammar = "does";
            print!("err: ");
            if lhs {
                print!("first argument of type \x1b[33m{}\x1b[0m with value of \x1b[33m{}\x1b[0m ", act1.get_type(), act1);
            }
            if lhs && rhs {
                do_grammar = "do";
                print!("and ")
            }
            if rhs {
                print!("second argument of type \x1b[33m{}\x1b[0m with value \x1b[33m{}\x1b[0m ", act2.get_type(), act2);
            }
            print!("{} not match constraints in the function \x1b[36m{}\x1b[0m, with signature (", do_grammar, op);
            if lhs {
                print!("\x1b[31m{}\x1b[0m, ", exp1)
            } else {
                print!("{}, ", exp1)
            }
            if rhs {
                print!("\x1b[31m{}\x1b[0m ", exp2)
            } else {
                print!("{} ", exp1)
            }
            println!("-> {})", op.get_signature().ret);
        }
        _ => {}
    }
}

enum Args {
    Unary(Parsed),
    Binary(Parsed, Parsed),
    //Temary(Type, Type, Type)
}


fn get_closures (expected: Params, input: &mut VecDeque<Parsed>) -> Result<Modifiers, StackError> {
    match expected {
        Params::Nullary => {
            Ok(Modifiers::None)
        },
        Params::Unary(constraint) => {
            if let Some(val) = input.pop_front() {
                if constraint.is_satisfied_by(&val.get_type()) {
                    Ok(Modifiers::Unary(val))
                } else {
                    //TODO: Stack error
                    Err(StackError::Undefined)
                }
            } else {
                //TODO: Stack error
                Err(StackError::Undefined)
            }
        },
        Params::Binary(c1, c2) => {
            let arg1 = input.pop_front();
            let arg2 = input.pop_front();
            match (arg1, arg2) {
                (Some(val), Some(val2)) => {
                    if c1.is_satisfied_by(&val.get_type()) &&
                        c2.is_satisfied_by(&val2.get_type()) {
                        Ok(Modifiers::Binary(val, val2))
                    } else {
                        //TODO:: STack Error
                        Err(StackError::Undefined)
                    }
                },
                //TODO: Stack error
                _ => Err(StackError::Undefined)
            }
        },
        _ => panic!("closure arguments defined for max 2 quotations")

    }
}








