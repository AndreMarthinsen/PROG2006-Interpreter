use std::cmp::Ordering;
use std::env::Args;
use std::fmt;
use num::traits::CheckedAdd;
use std::fmt::{Debug, Display, Error, Formatter, write};
use std::ops::{Add, BitAnd, BitOr, Div, Mul, Sub};



//////////////////// PARSED       //////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
/// enumeration of stack values, allowing the stack to hold
/// arbitrary types.
pub enum Parsed {
    Num(Numeric),
    String(String),
    Boolean(bool),
    Block(Vec<Parsed>),
    Binding(String),
    List(Vec<Parsed>),
    Error(StackError),
    Operation(Op),
}

/// Implements Add for StackTokens, with varying behaviour depending on the type.
impl<'a, 'b> Add<&'b Parsed> for &'a Parsed { //impl<'a, 'b> Add<&'b Numeric> for &'a Numeric
    type Output = Parsed;

    fn add(self, rhs: &'b Parsed) -> Self::Output {
        match (self, rhs) {
            (Parsed::Num(v1), Parsed::Num(v2)) => Parsed::Num(v1 + v2),
            (Parsed::String(s), Parsed::String(s2)) => {
                Parsed::String(s.clone().add(&s2))
            },
            (Parsed::List(l1), Parsed::List(l2)) => {
                let mut l1c = l1.clone();
                let mut l2c = l2.clone();
                l1c.append(&mut l2c);
                Parsed::List(l1c)
            }
            (_, Parsed::Num(_)) => Parsed::Error(StackError::InvalidLeft),
            (_, Parsed::String(_)) => Parsed::Error(StackError::InvalidLeft),
            (Parsed::Num(_), _) => Parsed::Error(StackError::InvalidRight),
            (Parsed::String(_), _) => Parsed::Error(StackError::InvalidRight),
            (_, _) => Parsed::Error(StackError::InvalidBoth)
        }
    }
}

/// Implements Sub for StackTokens, with varying behaviour depending on the type.
impl<'a, 'b> Sub<&'b Parsed> for &'a Parsed { //impl<'a, 'b> Add<&'b Numeric> for &'a Numeric
type Output = Parsed;

    fn sub(self, rhs: &'b Parsed) -> Self::Output {
        match (self, rhs) {
            (Parsed::Num(v1), Parsed::Num(v2)) => Parsed::Num(v1 - v2),
            (_, Parsed::Num(_)) => Parsed::Error(StackError::InvalidLeft),
            (Parsed::Num(_), _) => Parsed::Error(StackError::InvalidRight),
            (_, _) => Parsed::Error(StackError::InvalidBoth)
        }
    }
}

/// Implements Mul for StackTokens, with varying behaviour depending on the type.
impl<'a, 'b> Mul<&'b Parsed> for &'a Parsed { //impl<'a, 'b> Add<&'b Numeric> for &'a Numeric
type Output = Parsed;

    fn mul(self, rhs: &'b Parsed) -> Self::Output {
        match (self, rhs) {
            (Parsed::Num(v1), Parsed::Num(v2)) => Parsed::Num(v1 * v2),
            (_, Parsed::Num(_)) => Parsed::Error(StackError::InvalidLeft),
            (Parsed::Num(_), _) => Parsed::Error(StackError::InvalidRight),
            (_, _) => Parsed::Error(StackError::InvalidBoth)
        }
    }
}

/// Implements Div for StackTokens, with varying behaviour depending on the type.
impl<'a, 'b> Div<&'b Parsed> for &'a Parsed { //impl<'a, 'b> Add<&'b Numeric> for &'a Numeric
type Output = Parsed;

    fn div(self, rhs: &'b Parsed) -> Self::Output {
        match (self, rhs) {
            (Parsed::Num(v1), Parsed::Num(v2)) => Parsed::Num(v1 / v2),
            (_, Parsed::Num(_)) => Parsed::Error(StackError::InvalidLeft),
            (Parsed::Num(_), _) => Parsed::Error(StackError::InvalidRight),
            (_, _) => Parsed::Error(StackError::InvalidBoth)
        }
    }
}

impl Parsed {
    /// Defines what can be StackToken variants can interpreted as true,
    /// and under which conditions they are considered true.
    fn is_true(&self) -> bool {
        match self {
            Parsed::Num(val) => *val != Numeric::Int32(0),
            Parsed::Boolean(val) => *val,
            Parsed::String(s) => !s.is_empty(),
            Parsed::List(l) => !l.is_empty(),
            Parsed::Error(_) => false,
            _ => false,
        }
    }
}


/// Uses the bitwise and operator as a shorthand for logical AND.
impl<'a, 'b> BitAnd<&'b Parsed> for &'a Parsed {
    type Output = Parsed;

    fn bitand(self, rhs: &'b Parsed) -> Self::Output {
        Parsed::Boolean(self.is_true() && rhs.is_true())
    }
}

/// Uses the bitwise or operator as shorthand for logical OR.
impl<'a, 'b> BitOr<&'b Parsed> for &'a Parsed {
    type Output = Parsed;

    fn bitor(self, rhs: &'b Parsed) -> Self::Output {
        Parsed::Boolean(self.is_true() || rhs.is_true())
    }
}


impl PartialEq for Parsed {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Parsed::Num(v1), Parsed::Num(v2)) => v1 == v2,
            (Parsed::String(s1), Parsed::String(s2)) => s1.eq(s2),
            (Parsed::List(l1), Parsed::List(l2)) => l1.eq(l2),
            (Parsed::Boolean(b1), Parsed::Boolean(b2)) => b1 == b2,
            (Parsed::Error(err1), Parsed::Error(err2)) => err1 == err2,
            (_, _) => false
        }
    }
}


/// Implements Display for StackToken, allowing a pretty print of the
/// contents of a stack and in-program representation in general.
impl Display for Parsed {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Parsed::Error(err) => write!(f, "{}", err),
            Parsed::String(s) => write!(f, "\"{}\"", s),
            Parsed::Boolean(b) => write!(f, "{}", b),
            Parsed::Binding(s) => write!(f, "{}", s),
            Parsed::List(list) => {
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
            Parsed::Operation(op) => write!(f, "op: {:?}", op),
            Parsed::Block(c) => {
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
            Parsed::Num(n) => write!(f, "{}", n),
            _ => write!(f, "something else") //TODO: Error here?
        }
    }
}

//////////////////////////////// STACK ERROR //////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum StackError { // TODO: Keep as a single error type, or make specific variants?
    // Arithmetic errors
    Overflow,
    ZeroDiv,
    InvalidLeft,
    InvalidRight,
    InvalidBoth,
    // Operational errors
    PopEmpty,
    InternalBug
}

impl Display for StackError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StackError::Overflow => write!(f, "err: numeric overflow"),
            StackError::ZeroDiv=> write!(f, "err: zero division"),
            StackError::InvalidLeft => write!(f, "err: invalid left hand operand"),
            StackError::InvalidRight => write!(f, "err: invalid right hand operand"),
            StackError::InvalidBoth => write!(f, "err: operands not defined for function"),
            StackError::PopEmpty => write!(f, "err: attempted to pop empty stack!"),
            StackError::InternalBug => write!(f, "err: something has gone wrong!")
        }
    }
}





/////////////////////////// NUMERIC ///////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Copy)]
/// Numeric encapsulates numeric types such as integers and floats, implementing
/// basic arithmetic operations such as +, -, / and *.
pub enum Numeric {
    Int32(i32),
    Float64(f64),
    NumError(StackError)
}

impl Numeric {
    fn is_true(&self) -> bool {
        match self {
            Numeric::Int32(val) => *val != 0,
            Numeric::Float64(val) => *val != 0.0,
            Numeric::NumError(_) => false,
        }
    }

    fn as_i32(& self) -> Numeric {
        match self {
            Numeric::Float64(val) => Numeric::Int32(*val as i32),
            non_convertible => *non_convertible
        }
    }

    fn as_f64(& self) -> Numeric {
        match self {
            Numeric::Int32(val) => Numeric::Float64(*val as f64),
            non_convertible => *non_convertible
        }
    }
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

impl PartialOrd for Numeric {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.as_f64(), other.as_f64()) {
            (Numeric::Float64(v1), Numeric::Float64(v2)) => {
                v1.partial_cmp(&v2)
            },
            (_, _) => None
        }
    }

    fn lt(&self, other: &Self) -> bool {
        binary_numerical(self, other, try_lt).is_true()
    }

    fn le(&self, other: &Self) -> bool {
        binary_numerical(self, other, try_le).is_true()
    }

    fn gt(&self, other: &Self) -> bool {
        binary_numerical(self, other, try_gt).is_true()
    }

    fn ge(&self, other: &Self) -> bool {
        binary_numerical(self, other, try_ge).is_true()
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
impl<'a, 'b> Add<&'b Numeric> for &'a Numeric {
    type Output = Numeric;
    fn add(self, rhs: &'b Numeric) -> Self::Output {
        binary_numerical(self, rhs, try_add)
    }
}

/// Implements subtraction for the Numeric type. Int x Float operations
/// will result in Float variants being returned.
impl<'a, 'b> Sub<&'b Numeric> for &'a Numeric {
    type Output = Numeric;
    fn sub(self, rhs: &'b Numeric) -> Self::Output {
        binary_numerical(self, rhs, try_sub)
    }
}

/// Implements multiplication for the Numeric type. Int x Float operations
/// will result in Float variants being returned.
impl<'a, 'b> Mul<&'b Numeric> for &'a Numeric {
    type Output = Numeric;
    fn mul(self, rhs: &'b Numeric) -> Self::Output {
        binary_numerical(self, rhs, try_mul)
    }
}

/// Implements division for the Numeric type. Int x Float operations
/// will result in Float variants being returned.
impl<'a, 'b> Div<&'b Numeric> for &'a Numeric {
    type Output = Numeric;
    fn div(self, rhs: &'b Numeric) -> Self::Output {
        binary_numerical(self, rhs, try_div)
    }
}


/// binary_numerical encapsulates binary operations for the Numeric enum type,
/// allowing reduced repetition of pattern matching and error handling.
fn binary_numerical(lhs: &Numeric, rhs: &Numeric, op: fn(f64, f64) ->Result<f64, StackError>) -> Numeric {
    match (lhs, rhs) {
        (Numeric::NumError(err), _) => Numeric::NumError(err.clone()),
        (_, Numeric::NumError(err)) => Numeric::NumError(err.clone()),
        (Numeric::Int32(v1), Numeric::Int32(v2)) => {
            match op(*v1 as f64, *v2 as f64) {
                Ok(val) => Numeric::Int32(val as i32),
                Err(e) => Numeric::NumError(e)
            }
        },
        (mut left, mut right) => {
            match (left.as_f64(), right.as_f64()) {
                (Numeric::Float64(v1), Numeric::Float64(v2)) => {
                    match op(v1, v2) {
                        Ok(val) => Numeric::Float64(val),
                        Err(e) => Numeric::NumError(e)
                    }
                },
                (_, _)=> Numeric::NumError(StackError::InternalBug)
            }
        }
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

fn try_lt(a: f64, b:f64) -> Result<f64, StackError> {
    Ok((a < b) as i32 as f64)
}

fn try_le(a: f64, b:f64) -> Result<f64, StackError> {
    Ok((a <= b) as i32 as f64)
}

fn try_gt(a: f64, b:f64) -> Result<f64, StackError> {
    Ok((a > b) as i32 as f64)
}

fn try_ge(a: f64, b:f64) -> Result<f64, StackError> {
    Ok((a >= b) as i32 as f64)
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


