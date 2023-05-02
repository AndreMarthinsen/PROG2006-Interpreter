use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
/// enumeration of stack values, allowing the stack to hold
/// arbitrary types.
pub enum StackToken {
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Block(Vec<StackToken>),
    Binding(String),
    List(Vec<StackToken>),
    Err(String),
    Operation(Op),
    Empty
}

impl PartialEq for StackToken {
    fn eq(&self, other: &Self) -> bool {
        return match (self, other) {
            (StackToken::Integer(_), StackToken::Integer(_)) => true,
            (StackToken::Float(_), StackToken::Float(_)) => true,
            (StackToken::String(_), StackToken::String(_)) => true,
            (StackToken::Boolean(_), StackToken::Boolean(_)) => true,
            (StackToken::Block(_), StackToken::Block(_)) => true,
            (StackToken::Binding(_), StackToken::Binding(_)) => true,
            (StackToken::List(_), StackToken::List(_)) => true,
            (StackToken::Err(_), StackToken::Err(_)) => true,
            (StackToken::Operation(_), StackToken::Operation(_)) => true,
            (StackToken::Empty, StackToken::Empty) => true,
            (_, _) => false

        }
    }
}


/// enumerator of operations, i.e. specific functions.
pub enum Op {
    Void
}


impl Clone for Op {
    fn clone(&self) -> Self {
        match self {
            Op::Void => Op::Void,
//            Operation::Arithmetic(f) => Operation::Arithmetic(),
        }
    }
}




/// Display for Operations
impl Debug for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Op::Void => write!(f, "Void"),
            //  Op::Arithmetic(_) => write!(f, "Arithmetic"),
        }
    }
}


/// Implements Display for StackToken, allowing a pretty print of the
/// contents of a stack and in-program representation in general.
impl Display for StackToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StackToken::Empty => write!(f, "empty"),
            StackToken::Integer(i) => write!(f, "{}", i),
            StackToken::Float(fl) => write!(f, "{}", fl),
            StackToken::String(s) => write!(f, "\"{}\"", s),
            StackToken::Boolean(b) => write!(f, "{}", b),
            StackToken::Binding(s) => write!(f, "{}", s),
            StackToken::List(list) => {
                write!(f, "[")?;
                let mut iter = list.iter();
                if let Some(first) = iter.next() {
                    write!(f, "{}", first)?;
                    for item in iter {
                        write!(f, " {}", item)?;
                    }
                }
                write!(f, "]")
            },
            StackToken::Err(s) => write!(f, "ERROR: {}", s),
            StackToken::Operation(op) => write!(f, "op: {:?}", op),
            StackToken::Block(c) => {
                write!(f, "{{ ")?;
                let mut iter = c.iter();
                if let Some(first) = iter.next() {
                    write!(f, "{}", first)?;
                    for item in iter {
                        write!(f, " {}", item)?;
                    }
                }
                write!(f, " }}")
            },
        }
    }
}