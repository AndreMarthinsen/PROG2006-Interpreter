use std::fmt;
use std::fmt::{Display, Formatter, write};

#[derive(Clone, Debug, PartialEq, Copy)]
/// StackError represents various computational errors that can occur during program
/// run-time, such as overflows, mismatched operands, popping of an empty stack, and others.
pub enum StackError { // TODO: Keep as a single error type, or make specific variants?
// Arithmetic errors
Overflow,
    ZeroDiv,
    InvalidLeft,
    InvalidRight,
    InvalidBoth,
    // Operational errors
    PopEmpty,
    InternalBug,
    InvalidCoercion,
    Undefined,
}

/// Implements Display for StackError, writing "err: <specific error message>"
/// for each StackError variant.
impl Display for StackError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StackError::Overflow => write!(f, "err: numeric overflow"),
            StackError::ZeroDiv=> write!(f, "err: zero division"),
            StackError::InvalidLeft => write!(f, "err: invalid left hand operand"),
            StackError::InvalidRight => write!(f, "err: invalid right hand operand"),
            StackError::InvalidBoth => write!(f, "err: operands not defined for function"),
            StackError::PopEmpty => write!(f, "err: attempted to pop empty stack!"),
            StackError::InternalBug => write!(f, "err: something has gone wrong!"),
            StackError::InvalidCoercion => write!(f, "err: cannot coerce operands to target type"),
            StackError::Undefined => write!(f, "ERROR MESSAGE PLACEHOLDER")
        }
    }
}




