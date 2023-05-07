/////////////////////////// NUMERIC ///////////////////////////////////////////////////////////////

use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;
use crate::stack_error::StackError;

#[derive(Clone, Debug, Copy)]
/// Numeric encapsulates numeric types such as integers and floats, implementing
/// basic arithmetic operations such as +, -, / and *.
pub enum Numeric {
    Integer(i128),
    Float(f64),
    NumError(StackError)
}

impl Neg for Numeric {
    type Output = Numeric;

    fn neg(self) -> Self::Output {
        match self {
            Numeric::Integer(v) => Numeric::Integer(-v),
            Numeric::Float(v) => Numeric::Float(-v),
            Numeric::NumError(StackError) => self.clone()
        }
    }
}

// Numeric methods
impl Numeric {

    /// Allows implicit conversion from numeric to boolean by considering any
    /// non zero values as true.
    ///
    /// # Safety
    ///
    /// This function returns false for NumErrors, meaning its use gives no
    /// indication whether self is a valid numeric representation or not.
    fn is_true(&self) -> bool {
        match self {
            Numeric::Integer(val) => *val != 0,
            Numeric::Float(val) => *val != 0.0,
            Numeric::NumError(_) => false,
        }
    }

    /// Attempts to return self converted from any enum variant to Numeric::Int.
    /// If the type cannot be converted to Int, it returns itself.
    pub fn as_integer(& self) -> Numeric {
        match self {
            Numeric::Float(val) => Numeric::Integer(*val as i128),
            non_convertible => *non_convertible
        }
    }

    /// Attempts to return self converted from any enum variant to Numeric::Float.
    /// If the type cannot be converted to Int, it returns itself.
    pub fn as_float(& self) -> Numeric {
        match self {
            Numeric::Integer(val) => Numeric::Float(*val as f64),
            non_convertible => *non_convertible
        }
    }
}

/// Implements Display for the Numeric enum type.
/// Floats are always displayed with at least 1 precision.
impl Display for Numeric {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Numeric::Integer(v) => write!(f, "{}", v),
            Numeric::Float(v)=> {
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

/// Implements PartialOrd for Numeric. Relies on binary_numerical with
/// the try_* lt, le, gt and ge functions for actual logic.
///
/// Note that partial_cmp returns an Ordering enum only if the types match
/// in a way that allows implicit conversion, such as int int or float int.
impl PartialOrd for Numeric {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.as_float(), other.as_float()) {
            (Numeric::Float(v1), Numeric::Float(v2)) => {
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
///
/// # Usage
///
/// Can be used to check if two values are the same, but it can also be used to check
/// for specific NumErrors.
///
/// # Safety
///
/// Makes use of type conversion, and for large values this may not always work as expected
/// due to floating point precision.
///
impl PartialEq for Numeric {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Numeric::NumError(err), Numeric::NumError(err2)) => {
                err == err2
            }
            (Numeric::Integer(v), Numeric::Integer(v2)) => {
                v == v2
            }
            (Numeric::Float(v), Numeric::Integer(v2)) => {
                *v == *v2 as f64
            },
            (Numeric::Integer(v), Numeric::Float(v2)) => {
                *v as f64 == *v2
            },
            (Numeric::Float(v), Numeric::Float(v2)) => {
                v == v2
            }
            _ => false
        }
    }
}

/// Implements FromStr for Numeric for parsing directly from string.
impl FromStr for Numeric {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = s.parse::<i128>() {
            return Ok(Numeric::Integer(val));
        }
        if let Ok(val) = s.parse::<f64>() {
            return Ok(Numeric::Float(val));
        }
        Err("not parsable as Numeric".to_string())
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


/// binary_numerical is a function that encapsulates binary operations for the Numeric enum type.
///
/// # Arguments
///
/// * `lhs` - Left hand operand
///
/// * `rhs`- Right hand operand
///
/// * `op`- a binary function expecting two f64 values
// TODO: Add specific op for int int operations to avoid problems with precision for i128?
fn binary_numerical(lhs: &Numeric, rhs: &Numeric, op: fn(f64, f64) ->Result<f64, StackError>) -> Numeric {
    match (lhs, rhs) {
        (Numeric::NumError(err), _) => Numeric::NumError(err.clone()),
        (_, Numeric::NumError(err)) => Numeric::NumError(err.clone()),
        (Numeric::Integer(v1), Numeric::Integer(v2)) => {
            match op(*v1 as f64, *v2 as f64) {
                Ok(val) => Numeric::Integer(val as i128),
                Err(e) => Numeric::NumError(e)
            }
        },
        (mut left, mut right) => {
            match (left.as_float(), right.as_float()) {
                (Numeric::Float(v1), Numeric::Float(v2)) => {
                    match op(v1, v2) {
                        Ok(val) => Numeric::Float(val),
                        Err(e) => Numeric::NumError(e)
                    }
                },
                // Should never occur if type system is properly set up.
                (_, _)=> panic!("Encountered undefined type mismatch in binary_numerical.")
            }
        }
    }
}

/// try_add is a function that adds two f64 numbers together and returns the result as a Result<f64, StackError>.
/// It handles any errors related to the arithmetic operation, returning a StackError in the event of a failed operation.
fn try_add(a: f64, b: f64) -> Result<f64, StackError> {
    Ok(a + b)
}

/// try_sub is a function that subtracts two f64 numbers and returns the result as a Result<f64, StackError>.
/// It handles any errors related to the arithmetic operation, returning a StackError in the event of a failed operation.
fn try_sub(a: f64, b: f64) -> Result<f64, StackError> {
    Ok(a - b)
}

/// try_mul is a function that multiplies two f64 numbers together and returns the result as a Result<f64, StackError>.
/// It handles any errors related to the arithmetic operation, returning a StackError in the event of a failed operation.
fn try_mul(a: f64, b: f64) -> Result<f64, StackError> {
    Ok(a * b)
}

/// try_div is a function that divides two f64 numbers and returns the result as a Result<f64, StackError>.
/// It handles any errors related to the arithmetic operation, returning a StackError with the ZeroDiv variant in the event of a zero divisor.
fn try_div(a: f64, b: f64) -> Result<f64, StackError> {
    if b != 0.0 {
        Ok(a / b)
    } else {
        Err(StackError::ZeroDiv)
    }
}

/// try_lt is a function that compares two f64 numbers and returns 1.0 if the first is less than the second, and 0.0 otherwise.
/// It handles any errors related to the logical operation, returning a StackError in the event of a failed operation.
fn try_lt(a: f64, b: f64) -> Result<f64, StackError> {
    Ok((a < b) as i32 as f64)
}

/// try_le is a function that compares two f64 numbers and returns 1.0 if the first is less than or equal to the second, and 0.0 otherwise.
/// It handles any errors related to the logical operation, returning a StackError in the event of a failed operation.
fn try_le(a: f64, b: f64) -> Result<f64, StackError> {
    Ok((a <= b) as i32 as f64)
}

/// try_gt is a function that compares two f64 numbers and returns 1.0 if the first is greater than the second, and 0.0 otherwise.
/// It handles any errors related to the logical operation, returning a StackError in the event of a failed operation.
fn try_gt(a: f64, b: f64) -> Result<f64, StackError> {
    Ok((a > b) as i32 as f64)
}

/// try_ge is a function that compares two f64 numbers and returns 1.0 if the first is greater than or equal to the second, and 0.0 otherwise.
/// It handles any errors related to the logical operation, returning a StackError in the event of a failed operation.
fn try_ge(a: f64, b: f64) -> Result<f64, StackError> {
    Ok((a >= b) as i32 as f64)
}
