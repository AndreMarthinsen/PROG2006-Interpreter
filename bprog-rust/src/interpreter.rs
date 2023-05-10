use std::collections::{HashMap, VecDeque};
use crate::op::{Op};
use crate::parsed::Parsed;
use crate::stack::Stack;
use crate::stack_error::{arg_mismatch, StackError};
use crate::types::{Params};



pub struct Binding {
    pub function: bool,
    pub constant: bool,
    pub value: Parsed
}



pub enum Args {
    Nullary,
    Unary(Parsed),
    Binary(Parsed, Parsed),
    //Temary(Type, Type, Type)
}



pub fn run(stack: &mut Stack<Parsed>, input: &mut VecDeque<Parsed>, bindings: &mut HashMap<String, Binding>, fatal: bool) {
    while let Some(p) = input.pop_front() {
        match p.clone() {
            Parsed::Error(_) => {
                stack.push(p);
                break;
            }
            Parsed::Symbol(s) => {
                if let Some (val) = bindings.get(&s) {
                    if val.function {
                        run(stack, &mut VecDeque::from(val.value.get_contents().unwrap()), bindings, fatal)
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
                exec_op(&op, stack, input, bindings, fatal)
            },
            other => {
                stack.push(other)
            }
        }
        if let Some(Parsed::Error(err)) = stack.top() {
            if fatal { panic!("{}", err)} else { println!("{}", err)}
            stack.clear();
            stack.push(p);
            break;
        }
    }
}




fn exec_op(op: &Op, stack: &mut Stack<Parsed>, input: &mut VecDeque<Parsed>, bindings: &mut HashMap<String, Binding>, fatal: bool) {
    let signature = op.clone().get_signature();
    let mut arg  = Parsed::Error(StackError::PopEmpty);
    let mut arg2 = Parsed::Error(StackError::PopEmpty);
    let mods: Args;
    let ret;
    match get_modifiers(&op, input, bindings) {
        Ok(m) => mods = m,
        Err(e) => {stack.push(Parsed::Error(e)); return;},
    }

    if let Params::Unary(_) | Params::Binary(_, _) = &signature.stack_args {
        if let Some(t) = stack.pop() {
            arg = t
        } else {
            stack.push(Parsed::Error(StackError::PopEmpty));
            return;
        }
    }
    if let Params::Binary(_, _) = &signature.stack_args {
        if let Some(t) = stack.pop() {
            arg2 = t
        } else {
            stack.push(Parsed::Error(StackError::PopEmpty));
            return;
        }
    }
    match &signature.stack_args {
        Params::Nullary => {
            ret = op.exec_nullary(mods, bindings);
        },
        Params::Unary(c) => {
            ret = if !c.is_satisfied_by(&arg.get_type()) {
                Parsed::Error(arg_mismatch(&op, &Args::Unary(arg), true))
            } else {
                op.exec_unary(arg, mods, bindings)
            }
        },
        Params::Binary(c1, c2) => {
            // Checks that the constraints of the function signature is satisfied.
            ret = if !c1.is_satisfied_by(&arg2.get_type()) ||
                !c2.is_satisfied_by(&arg.get_type()) {
                Parsed::Error(
                         arg_mismatch(&op, &Args::Binary(arg2.clone(), arg.clone()), true))
            } else {
                op.exec_binary(&arg2, &arg, mods, bindings)
            }
        },
        _ => panic!("temary arguments not implemented")
    }

    match ret {
        Parsed::Quotation(q) => {
            run(stack, &mut q.clone(), bindings, fatal)
        },
        Parsed::Void => {},
        _ => stack.push(ret)
    }
}



fn get_modifiers(op: &Op, input: &mut VecDeque<Parsed>, bindings: &mut HashMap<String, Binding>)
    -> Result<Args, StackError> {
    let expected = op.get_signature().modifiers;

    let mut mod1 = Parsed::Error(StackError::Undefined);
    let mut mod2 = Parsed::Error(StackError::Undefined);
    let mods;
    if let Params::Unary(_) | Params::Binary(_,_) = expected {
        if let Some( m) = input.pop_front() {
            mod1 = if op.clone() != Op::AsSymbol {
                resolve_symbol(m, bindings)
            } else {
                m
            }
        } else {
            return Err(StackError::PrematureEnd)
        }
    }
    if let Params::Binary(_, _) = expected {
        if let Some( m) = input.pop_front() {
            mod2 = if op.clone() != Op::AsSymbol {
                resolve_symbol(m, bindings)
            } else {
                m
            }
        } else {
            return Err(StackError::PrematureEnd)
        }
    }
    match expected {
        Params::Nullary => mods = Ok(Args::Nullary),
        Params::Unary(constraint) => {
            if constraint.is_satisfied_by(&mod1.get_type()) {
                mods = Ok(Args::Unary(mod1));
            } else {
                return Err(arg_mismatch(&op, &Args::Unary(mod1), false));
            }
        },
        Params::Binary(c1, c2) => {
            if c1.is_satisfied_by(&mod1.get_type()) &&
                c2.is_satisfied_by(&mod2.get_type()) {
                mods = Ok(Args::Binary(mod1, mod2));
            } else {
                return Err(arg_mismatch(&op, &Args::Binary(mod1, mod2), false));
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





