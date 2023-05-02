use std::arch::asm;
use std::env::Args;
use std::fmt;
use num::traits::CheckedAdd;
use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug)]
/// enumeration of stack values, allowing the stack to hold
/// arbitrary types.
pub enum StackToken {
    Num(Numeric),
    String(String),
    Boolean(bool),
    Block(Vec<StackToken>),
    Binding(String),
    List(Vec<StackToken>),
    Error(String),
    Operation(Op),
    Empty
}

pub enum StackError {
    Overflow,
    ZeroDiv
}

impl Add for StackToken {
    type Output = StackToken;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (StackToken::Num(v1), StackToken::Num(v2)) => StackToken::Num(v1 + v2),
            (_, _) => StackToken::Error("not valid operation".to_string())
        }
    }
}

#[derive(Clone, Debug)]
pub enum Numeric {
    Int32(i32),
    Float64(f64),
}

impl Display for Numeric {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Numeric::Int32(v) => write!(f, "{}", v),
            Numeric::Float64(v)=> {
                if v.fract() == 0.0 {
                    write!(f, "{}.0", v.to_string())
                } else {
                    write!(f, "{}", v.to_string())
                }
            }
        }
    }
}



impl Add for Numeric {
    type Output = Numeric;

    fn add(self, rhs: Self) -> Self::Output {
        binary_numerical(self, rhs, try_add)
    }


}

impl Sub for Numeric {
    type Output = Numeric;

    fn sub(self, rhs: Self) -> Self::Output {
        binary_numerical(self, rhs, try_sub)
    }
}

impl Mul for Numeric {
    type Output = Numeric;

    fn mul(self, rhs: Self) -> Self::Output {
        binary_numerical(self, rhs, try_mul)
    }
}

impl Div for Numeric {
    type Output = Numeric;

    fn div(self, rhs: Self) -> Self::Output {
        binary_numerical(self, rhs, try_div)
    }
}


fn binary_numerical(lhs: Numeric, rhs: Numeric, op: fn(f64, f64) ->Result<f64, StackError>) -> Numeric {
    match (lhs, rhs) {
        (Numeric::Int32(v1), Numeric::Int32(v2)) => {
            match op(v1 as f64, v2 as f64) {
                Ok(val) => Numeric::Int32(val as i32),
                _ => Numeric::Int32(0)
            }
        },
        (Numeric::Float64(v1), Numeric::Int32(v2)) => {
            match op(v1, v2 as f64) {
                Ok(val) => Numeric::Float64(val),
                _ => Numeric::Int32(0)
            }
        },
        (Numeric::Int32(v1), Numeric::Float64(v2)) => {
            match op(v1 as f64, v2) {
                Ok(val) => Numeric::Float64(val),
                _ => Numeric::Int32(0)
            }
        },
        (Numeric::Float64(v1), Numeric::Float64(v2)) => {
            match op(v1, v2) {
                Ok(val) => Numeric::Float64(val),
                _ => Numeric::Int32(0)
            }
        },
    }
}

fn try_add(a: f64, b: f64) -> Result<f64, StackError> {
    Ok(a + b)
}

fn try_sub(a: f64, b: f64) -> Result<f64, StackError> {
    Ok(a - b)
}

fn try_mul(a: f64, b: f64) -> Result<f64, StackError> {
    Ok(a * b)
}

fn try_div(a: f64, b: f64) -> Result<f64, StackError> {
    if b != 0.0 {
        Ok(a / b)
    } else {
        Err(StackError::ZeroDiv)
    }
}


/*
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
*/

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
            _ => write!(f, "")
        }
    }
}