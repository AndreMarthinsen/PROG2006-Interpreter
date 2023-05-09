use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use crate::op::Op;
use crate::types::Params;
use crate::interpreter::{Args};

#[derive(Clone, Debug, PartialEq)]
/// StackError represents various computational errors that can occur during program
/// run-time, such as overflows, mismatched operands, popping of an empty stack, and others.
pub enum StackError { // TODO: Keep as a single error type, or make specific variants?
    // Arithmetic errors
    Overflow,
    ZeroDiv,
    // Operational errors
    InvalidCoercion,
    Undefined,
    // Constraint errors
    TypeMismatch(String),

    // List errors
    HeadEmpty,
    TailEmpty,

    // Stack errors
    PopEmpty,
    PrematureEnd,

    // Others
    UserDefined(String),
}

/// Implements Display for StackError, writing "err: <specific error message>"
/// for each StackError variant.
impl Display for StackError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StackError::Overflow => write!(f, "err: numeric overflow"),
            StackError::ZeroDiv=> write!(f, "err: zero division"),
            StackError::PopEmpty => write!(f, "err: attempted to pop empty stack!"),
            StackError::PrematureEnd => write!(f, "expected more program input, but none was found."),
            StackError::InvalidCoercion => write!(f, "err: cannot coerce operands to target type"),
            StackError::Undefined => write!(f, "ERROR MESSAGE PLACEHOLDER"),
            StackError::TypeMismatch(s) => write!(f, "{}", s),
            StackError::UserDefined(s) => write!(f, "{}", s),
            _ => write!(f, "not implemented")
        }
    }
}


pub fn arg_mismatch(op: &Op, got: &Args, stack_arg: bool) -> StackError {
    let exp = if stack_arg {
        op.get_signature().stack_args
    } else {
        op.get_signature().modifiers
    };
    StackError::TypeMismatch(match (exp.clone(), got.clone()) {
        (Params::Unary(expected), Args::Unary(actual)) => {
            let s = format!("err: argument of type \x1b[33m{}\x1b[0m with value \x1b[33m{}\x1b[0m does \
             not satisfy constraint in the function \x1b[36m{}\x1b[0m, with signature", actual.get_type(), actual, op);
            s.add(
                & if stack_arg {
                    format!(" <(\x1b[31m{}\x1b[0m)::({}) -> {}>.", expected, op.get_signature().modifiers, op.get_signature().ret)
                } else {
                    format!(" <({})::(\x1b[31m{}\x1b[0m) -> {}>.", op.get_signature().stack_args, expected, op.get_signature().ret)
                }
            )
        },
        (Params::Binary(exp1, exp2), Args::Binary(act1, act2)) => {
            let (lhs, rhs) =
                (!exp1.is_satisfied_by(&act1.get_type()),
                 !exp2.is_satisfied_by(&act2.get_type()));
            let mut err_msg = "err: ".to_string();
            if lhs {
                err_msg += &format!("first argument of type \x1b[33m{}\x1b[0m with \
                 value of \x1b[33m{}\x1b[0m ", act1.get_type(), act1);
            }
            if lhs && rhs {
                err_msg += "and ";
            }
            if rhs {
                err_msg += &format!("second argument of type \x1b[33m{}\x1b[0m with \
                 value \x1b[33m{}\x1b[0m ", act2.get_type(), act2);
            }

            err_msg += &format!("{} not match constraints in the function \x1b[36m{}\x1b[0m, \
             with signature ", if lhs && rhs { "do" } else { "does" }, op);
            let first = if lhs {
                format!("\x1b[31m{}\x1b[0m", exp1)
            } else {
                format!("{}", exp1)
            };
            let second = if rhs {
                format!("\x1b[31m{}\x1b[0m", exp2)
            } else {
                format!("{}", exp2)
            };
            err_msg += &if stack_arg {
                format!("<({}, {})::({}) -> {})>", first, second, op.get_signature().modifiers, op.get_signature().ret)
            } else {
                format!("<({})::({}, {}) -> {})>", op.get_signature().modifiers, first, second, op.get_signature().ret)
            };
            err_msg
        },
        _ => "".to_string(),
    })
}
