use std::collections::VecDeque;
use crate::op::{Op, Closures};
use crate::parsed::Parsed;
use crate::stack::Stack;
use crate::stack_error::StackError;
use crate::types::{Params, Constraint, Type};

pub fn run(stack: &mut Stack<Parsed>, input: &mut VecDeque<Parsed>) {
    while !input.is_empty() {
        if let Some(p) = input.pop_front() {
            match p {
                Parsed::Function(op) => {
                    exec_op(op, stack, input )
                },
                other => {
                    stack.push(other)
                }
            }
        }
    }
}



fn exec_op(op: Op, stack: &mut Stack<Parsed>, input: &mut VecDeque<Parsed>) {
    let signature = op.get_signature();
    match &signature.stack_args {
        Params::Nullary => {

            if signature.ret != Constraint::Void {
                stack.push(op.exec_nullary(Closures::None))
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
                                    run(stack, &mut q.clone())
                                },
                                Parsed::Void => {},
                                _ => stack.push(res)
                            }
                        } else {
                            println!("{} {} {}", res, res.get_type(), signature.ret);
                        };
                    } else {
                        print_mismatch_arg(op, signature.stack_args, Args::Unary(arg.get_type()))
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
                    let res = op.exec_binary(&lhs, &rhs, closures);
                    if signature.ret.is_satisfied_by(&res.get_type()) {
                        match res {
                            Parsed::Quotation(q) => {
                                run(stack, &mut q.clone())
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
                    lhs.get_type(),
                    rhs.get_type())
                )
            }
        },
        _ => {}
    }
}

fn print_mismatch_arg(op: Op, exp: Params, got: Args) {
    match (exp, got) {
        (Params::Unary(expected), Args::Unary(actual)) => {
            println!("err: argument of type \x1b[33m{}\x1b[0m does not satisfy constraint in \
            the function \x1b[36m{}\x1b[0m, with signature (\x1b[31m{}\x1b[0m -> {}).", actual, op, expected, op.get_signature().ret)
        },
        (Params::Binary(exp1, exp2), Args::Binary(act1, act2)) => {
            println!("bug: {} {}", act1, act2);
            let lhs = !exp1.is_satisfied_by(&act1);
            let rhs = !exp2.is_satisfied_by(&act2);
            let mut do_grammar = "does";
            print!("err: ");
            if lhs {
                print!("first argument of type \x1b[33m{}\x1b[0m ", act1);
            }
            if lhs && rhs {
                do_grammar = "do";
                print!("and ")
            }
            if rhs {
                print!("second argument of type \x1b[33m{}\x1b[0m ", act2);
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
    Unary(Type),
    Binary(Type, Type),
    //Temary(Type, Type, Type)
}


fn get_closures (expected: Params, input: &mut VecDeque<Parsed>) -> Result<Closures, StackError> {
    match expected {
        Params::Nullary => {
            Ok(Closures::None)
        },
        Params::Unary(constraint) => {
            if let Some(val) = input.pop_front() {
                let quot = val.coerce(&Type::Quotation);
                if constraint.is_satisfied_by(&quot.get_type()) {
                    Ok(Closures::Unary(val))
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
                    let quot1 = val.coerce(&Type::Quotation);
                    let quot2 = val.coerce( &Type::Quotation);
                    if c1.is_satisfied_by(&quot1.get_type()) &&
                        c2.is_satisfied_by(&quot2.get_type()) {

                        Ok(Closures::Binary(val, val2))
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








