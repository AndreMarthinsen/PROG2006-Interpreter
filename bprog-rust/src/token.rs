use std::arch::asm;
use std::env::Args;
use std::fmt;
use num::traits::CheckedAdd;
use std::fmt::{Debug, Display, Error, Formatter, write};
use std::ops::{Add, Div, Mul, Sub};


//////////////////// STACK TOKEN //////////////////////////////////////////////////////////////////

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

/// Implements Add for StackTokens, with varying behaviour depending on the type.
impl Add for StackToken {
    type Output = StackToken;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (StackToken::Num(v1), StackToken::Num(v2)) => StackToken::Num(v1 + v2),
            (StackToken::String(s), StackToken::String(s2)) => {
                StackToken::String(s.add(&s2))
            }
            (_, _) => StackToken::Error("not valid operation".to_string())
        }
    }
}

//////////////////////////////// STACK ERROR //////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq)]
pub enum StackError { // TODO: Keep as a single error type, or make specific variants?
    Overflow,
    ZeroDiv,
    PopEmpty
}

impl Display for StackError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StackError::Overflow => write!(f, "err: numeric overflow"),
            StackError::ZeroDiv=> write!(f, "err: zero division"),
            StackError::PopEmpty => write!(f, "err: attempted to pop empty stack!")
        }
    }
}





/////////////////////////// NUMERIC ///////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
/// Numeric encapsulates numeric types such as integers and floats, implementing
/// basic arithmetic operations such as +, -, / and *.
pub enum Numeric {
    Int32(i32),
    Float64(f64),
    NumError(StackError)
}

/// Implements Display for the Numeric enum type.
/// Floats are always displayed with at least 1 precision.
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
            Numeric::NumError(err) => write!(f, "{}", err)
        }
    }
}

/// Implements PartialEq for the Numeric type.
/// Allows implicit type conversion between numerical types such as i32 and f64,
/// allowing comparisons such as 5 == 5.0, which becomes 5.0 == 5.0
impl PartialEq for Numeric {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Numeric::NumError(err), Numeric::NumError(err2)) => {
                err == err2
            }
            (Numeric::Int32(v), Numeric::Int32(v2)) => {
                v == v2
            }
            (Numeric::Float64(v), Numeric::Int32(v2)) => {
                *v == *v2 as f64
            },
            (Numeric::Int32(v), Numeric::Float64(v2)) => {
                *v as f64 == *v2
            },
            (Numeric::Float64(v), Numeric::Float64(v2)) => {
                v == v2
            }
            _ => false
        }
    }
}


/// Implements addition for the Numeric type. Int x Float operations
/// will result in Float variants being returned.
impl Add for Numeric {
    type Output = Numeric;
    fn add(self, rhs: Self) -> Self::Output {
        binary_numerical(self, rhs, try_add)
    }
}

/// Implements subtraction for the Numeric type. Int x Float operations
/// will result in Float variants being returned.
impl Sub for Numeric {
    type Output = Numeric;
    fn sub(self, rhs: Self) -> Self::Output {
        binary_numerical(self, rhs, try_sub)
    }
}

/// Implements multiplication for the Numeric type. Int x Float operations
/// will result in Float variants being returned.
impl Mul for Numeric {
    type Output = Numeric;
    fn mul(self, rhs: Self) -> Self::Output {
        binary_numerical(self, rhs, try_mul)
    }
}

/// Implements division for the Numeric type. Int x Float operations
/// will result in Float variants being returned.
impl Div for Numeric {
    type Output = Numeric;
    fn div(self, rhs: Self) -> Self::Output {
        binary_numerical(self, rhs, try_div)
    }
}


/// binary_numerical encapsulates binary operations for the Numeric enum type,
/// allowing reduced repetition of pattern matching and error handling.
fn binary_numerical(lhs: Numeric, rhs: Numeric, op: fn(f64, f64) ->Result<f64, StackError>) -> Numeric {
    match (lhs, rhs) {
        (Numeric::Int32(v1), Numeric::Int32(v2)) => {
            match op(v1 as f64, v2 as f64) {
                Ok(val) => Numeric::Int32(val as i32),
                Err(e) => Numeric::NumError(e)
            }
        },
        (Numeric::Float64(v1), Numeric::Int32(v2)) => {
            match op(v1, v2 as f64) {
                Ok(val) => Numeric::Float64(val),
                Err(e) => Numeric::NumError(e)
            }
        },
        (Numeric::Int32(v1), Numeric::Float64(v2)) => {
            match op(v1 as f64, v2) {
                Ok(val) => Numeric::Float64(val),
                Err(e) => Numeric::NumError(e)
            }
        },
        (Numeric::Float64(v1), Numeric::Float64(v2)) => {
            match op(v1, v2) {
                Ok(val) => Numeric::Float64(val),
                Err(e) => Numeric::NumError(e)
            }
        },
        (Numeric::NumError(err), _) => Numeric::NumError(err),
        (_, Numeric::NumError(err)) => Numeric::NumError(err)
    }
}

// TODO: Error handling for these
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

/////////////////////////// OP ////////////////////////////////////////////////////////////////////

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