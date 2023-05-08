use std::collections::{HashMap, VecDeque};
use crate::op::{Op, Modifiers};
use crate::parsed::Parsed;
use crate::stack::Stack;
use crate::stack_error::StackError;
use crate::types::{Params};



pub struct Binding {
    pub function: bool,
    pub constant: bool,
    pub value: Parsed
}



enum Args {
    Unary(Parsed),
    Binary(Parsed, Parsed),
    //Temary(Type, Type, Type)
}



pub fn run(stack: &mut Stack<Parsed>, input: &mut VecDeque<Parsed>, bindings: &mut HashMap<String, Binding>) {
    while let Some(p) = input.pop_front() {
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
                stack.push(Parsed::List(s.iter()
                    .map(|p| resolve_symbol(p.clone(), bindings))
                    .collect()));
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




fn exec_op(op: Op, stack: &mut Stack<Parsed>, input: &mut VecDeque<Parsed>, bindings: &mut HashMap<String, Binding>) {
    let signature = op.clone().get_signature();
    let mut arg  = Parsed::Error(StackError::PopEmpty);
    let mut arg2 = Parsed::Error(StackError::PopEmpty);
    let mods: Modifiers;
    let ret;

    if let Ok(m) = get_modifiers(signature.modifiers, input, bindings) {
        mods = m
    } else {
        stack.push(Parsed::Error(StackError::Undefined));
        return
    }

    if let Params::Unary(_) | Params::Binary(_, _) = &signature.stack_args {
        arg = stack.pop().unwrap_or_else(|| Parsed::Error(StackError::PopEmpty));
    }
    if let Params::Binary(_, _) = &signature.stack_args {
        arg2 = stack.pop().unwrap_or_else(||Parsed::Error(StackError::PopEmpty));
    }
    match &signature.stack_args {
        Params::Nullary => {
            ret = op.exec_nullary(mods, bindings);
        },
        Params::Unary(c) => {
            if !c.is_satisfied_by(&arg.get_type()) {
                print_mismatch_arg(&op, signature.stack_args, &Args::Unary(arg));
                return;
            }
            ret = op.exec_unary(arg, mods, bindings);
        },
        Params::Binary(c1, c2) => {
            // Checks that the constraints of the function signature is satisfied.
            if !c1.is_satisfied_by(&arg2.get_type()) ||
                !c2.is_satisfied_by(&arg.get_type()) {
                print_mismatch_arg(&op, signature.stack_args, &Args::Binary(arg2.clone(), arg.clone()))
            }
            ret = op.exec_binary(&arg2, &arg, mods, bindings);
        },
        _ => panic!("temary arguments not implemented")
    }
    if signature.ret.is_satisfied_by(&ret.get_type()) {
        match ret {
            Parsed::Quotation(q) => {
                run(stack, &mut q.clone(), bindings)
            },
            Parsed::Void => {},
            _ => stack.push(ret)
        }} else {
        println!("{} {} {}", ret, ret.get_type(), signature.ret);
    };
}



fn get_modifiers(expected: Params, input: &mut VecDeque<Parsed>, bindings: &mut HashMap<String, Binding>)
    -> Result<Modifiers, StackError> {

    let mut mod1 = Parsed::Error(StackError::Undefined);
    let mut mod2 = Parsed::Error(StackError::Undefined);
    let mods;
    if let Params::Unary(_) | Params::Binary(_,_) = expected {
        if let Some( m) = input.pop_front() {
            mod1 = resolve_symbol(m, bindings);
        } else {
            return Err(StackError::PopEmpty)
        }
    }
    if let Params::Binary(_, _) = expected {
        if let Some( m) = input.pop_front() {
            mod2 = resolve_symbol(m, bindings);
        }else {
            return Err(StackError::PopEmpty)
        }
    }
    match expected {
        Params::Nullary => mods = Ok(Modifiers::None),
        Params::Unary(constraint) => {
            if constraint.is_satisfied_by(&mod1.get_type()) {
                mods = Ok(Modifiers::Unary(mod1))
            } else {
                mods = Err(StackError::Undefined)
            }
        },
        Params::Binary(c1, c2) => {
            if c1.is_satisfied_by(&mod1.get_type()) &&
                c2.is_satisfied_by(&mod2.get_type()) {
                mods = Ok(Modifiers::Binary(mod1, mod2))
            } else {
                mods = Err(StackError::Undefined)
            }
        },
        _ => panic!("closure arguments defined for max 2 quotations")
    }
    mods
}



fn resolve_symbol(sym: Parsed, bindings: &mut HashMap<String, Binding>) -> Parsed {
    match sym {
        Parsed::Symbol(s) => {
            if let Some (binding) = bindings.get(&s) {
                binding.value.clone()
            } else {
                Parsed::Symbol(s.clone())
            }
        },
        _ => sym
    }
}




fn print_mismatch_arg(op: &Op, exp: Params, got: &Args) {
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




