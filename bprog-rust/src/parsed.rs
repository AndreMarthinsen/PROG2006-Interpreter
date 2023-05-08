use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, BitAnd, BitOr, Div, Mul, Neg, Sub};
use crate::numeric::Numeric;
use crate::op::Op;
use crate::stack_error::StackError;
use crate::types::{numeric_coercion, Type};

//////////////////// PARSED       //////////////////////////////////////////////////////////////////

#[derive(Clone)]
/// enumeration of stack values, allowing a container to hold
/// arbitrary types.
pub enum Parsed {
    Void,
    Num(Numeric),
    String(String),
    Bool(bool),
    Quotation(VecDeque<Parsed>),
    Symbol(String),
    List(Vec<Parsed>),
    Error(StackError),
    Function(Op),
}

impl Neg for Parsed {
    type Output = Parsed;

    fn neg(self) -> Self::Output {
        match self {
            Parsed::Num(num) => Parsed::Num(-num),
            Parsed::Bool(b) => Parsed::Bool(!b),
            _ => Parsed::Error(StackError::InvalidRight)
        }
    }
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
            },
            (elem, Parsed::List(old_list)) => {
                let mut new_list:Vec<Parsed> = Vec::new();
                new_list.push(elem.clone());
                new_list.append(&mut old_list.clone());
                Parsed::List(new_list)
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

// Parsed methods
impl Parsed {
    /// Defines what can be StackToken variants can interpreted as true,
    /// and under which conditions they are considered true.
    fn is_true(&self) -> bool {
        match self {
            Parsed::Num(val) => *val != Numeric::Integer(0),
            Parsed::Bool(val) => *val,
            Parsed::String(s) => !s.is_empty(),
            Parsed::List(l) => !l.is_empty(),
            Parsed::Error(_) => false,
            _ => false,
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Parsed::Void => {
                Type::Void
            },
            Parsed::Num(numerical) => {
                match numerical {
                    Numeric::Integer(_) => Type::Integer,
                    Numeric::Float(_) => Type::Float,
                    Numeric::NumError(_) => Type::Error
                }
            }
            Parsed::String(_) => Type::String,
            Parsed::Bool(_) => Type::Bool,
            Parsed::Quotation(_) => Type::Quotation,
            Parsed::Symbol(_) => Type::Symbol,
            Parsed::List(_) => Type::List,
            Parsed::Error(_) => Type::Error,
            Parsed::Function(op) => Type::Function(op.get_signature())
        }
    }

    pub fn size (&self) -> Parsed {
        match self {
            Parsed::String(s) => Parsed::Num(Numeric::Integer(s.len() as i128)),
            Parsed::Quotation(b) => Parsed::Num(Numeric::Integer(b.len() as i128)),
            Parsed::List(l) => Parsed::Num(Numeric::Integer(l.len() as i128)),
            _ => Parsed::Error(StackError::InvalidLeft)
        }
    }


    pub fn get_contents(&self) -> Option<Vec<Parsed>>{
        match self {
            Parsed::List(l) => {
                Some(l.clone())
            },
            Parsed::Quotation(q) => {
                Some(Vec::from(q.clone()))
            }
            _ => None
        }
    }


    pub fn coerce(&self, t: &Type) -> Parsed {
        let res = match t {
            Type::Void => None,
            Type::String => self.to_string(),
            Type::List => self.to_list(),
            Type::Integer => self.to_integer(),
            Type::Float => self.to_float(),
            Type::Bool => self.to_bool(),
            Type::Quotation => self.to_quotation(),
            Type::Error => self.to_string(),
            Type::Symbol => self.to_symbol(),
            Type::Function(_) => self.to_function(),
        };
        if let Some(p) = res {
            p
        } else {
            Parsed::Error(StackError::InvalidCoercion)
        }
    }

    fn to_bool(&self) -> Option<Parsed> {
        match self {
            Parsed::Num(n) => {
                Some(Parsed::Bool(n != &Numeric::Integer(0)))
            }
            Parsed::Bool(_) => {
                Some(self.clone())
            },
            _ => None
        }
    }

    fn to_float(&self) -> Option<Parsed> {
        match self {
            Parsed::Bool(b) => {
                Some(Parsed::Num(Numeric::Float(*b as i32 as  f64)))
            },
            Parsed::Num(n) => {
                Some(Parsed::Num(n.as_float()))
            },
            _ => None
        }
    }

    fn to_integer(&self) -> Option<Parsed> {
        match self {
            Parsed::Bool(b) => {
                Some(Parsed::Num(Numeric::Integer(*b as i128)))
            },
            Parsed::Num(n) => {
                Some(Parsed::Num(n.as_integer()))
            },
            _ => None
        }
    }

    fn to_quotation(&self) -> Option<Parsed> {
        match self {
            Parsed::Quotation(_) => Some(self.clone()),
            Parsed::Error(_) => None,
            _ => {
                let mut deque = VecDeque::new();
                deque.push_back(self.clone());
                Some(Parsed::Quotation(deque))
            }

             //   Some(Parsed::Quotation(vec![self.clone()])),
        }
    }

    fn to_symbol(&self) -> Option<Parsed> {
        match self {
            Parsed::Symbol(_) => Some(self.clone()),
            _ => None,
        }
    }


    fn to_function(&self) -> Option<Parsed> {
        match self {
            Parsed::Function(_) => Some(self.clone()),
            _ => None,
        }
    }

    fn to_string(&self) -> Option<Parsed> {
        match self {
            Parsed::Error(_) => None,
            _ => {
                Some(Parsed::String(format!("{}", self)))
            }
        }
    }

    fn to_list(&self) -> Option<Parsed> {
        match self {
            Parsed::Error(_) => None,
            Parsed::List(_) => Some(self.clone()),
            _ => {
                Some(Parsed::List(vec![self.clone()]))
            }
        }
    }

}


/// Uses the bitwise and operator as a shorthand for logical AND.
impl<'a, 'b> BitAnd<&'b Parsed> for &'a Parsed {
    type Output = Parsed;

    fn bitand(self, rhs: &'b Parsed) -> Self::Output {
        Parsed::Bool(self.is_true() && rhs.is_true())
    }
}

/// Uses the bitwise or operator as shorthand for logical OR.
impl<'a, 'b> BitOr<&'b Parsed> for &'a Parsed {
    type Output = Parsed;

    fn bitor(self, rhs: &'b Parsed) -> Self::Output {
        Parsed::Bool(self.is_true() || rhs.is_true())
    }
}

/// Implements PartialEq for Parsed.
///
/// # Safety
///
/// Only the Num, String, List, Boolean and Error variants are properly equatable,
/// but as only a boolean is returned, there is no way to tell from usage of the function
/// alone to tell if the operation was legal to begin with.
///
impl PartialEq for Parsed {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Parsed::Num(v1), Parsed::Num(v2)) => v1 == v2,
            (Parsed::String(s1), Parsed::String(s2)) => s1.eq(s2),
            (Parsed::List(l1), Parsed::List(l2)) => l1.eq(l2),
            (Parsed::Bool(b1), Parsed::Bool(b2)) => b1 == b2,
            (Parsed::Error(err1), Parsed::Error(err2)) => err1 == err2,
            (Parsed::Quotation(q), Parsed::Quotation(q2)) => {
                q.eq(q2)
            }
            (_, _) => false
        }
    }
}

impl PartialOrd for Parsed {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Some(t) = numeric_coercion(&self.get_type(), &other.get_type()) {
            let coerced_l = self.coerce(&t);
            let coerced_r = other.coerce(&t);
            match (coerced_l, coerced_r) {
                (Parsed::Num(n), Parsed::Num(n2)) => {
                    n.partial_cmp(&n2)
                },
                _ => None
            }
        } else {
            None
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
            Parsed::Bool(b) => if *b {
                write!(f, "True")
            } else {
                write!(f, "False")
            },
            Parsed::Symbol(s) => write!(f, "{}", s),
            Parsed::List(list) => {
                write!(f, "[")?;
                let mut iter = list.iter();
                if let Some(first) = iter.next() {
                    write!(f, "{}", first)?;
                    for item in iter {
                        write!(f, ",{}", item)?;
                    }
                }
                write!(f, "]")
            },
            Parsed::Function(op) => write!(f, "{}", op),
            Parsed::Quotation(c) => {
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

/// Wraps Display for simplicity
impl Debug for Parsed {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Parsed::String(s) => write!(f, "\" {} \"", s),
            Parsed::List(list) => {
                write!(f, "[ ")?;
                let mut iter = list.iter();
                if let Some(first) = iter.next() {
                    write!(f, "{:?}", first)?;
                    for item in iter {
                        write!(f, " {:?}", item)?;
                    }
                }
                write!(f, " ]")
            },
            Parsed::Quotation(c) => {
                write!(f, "{{ ")?;
                let mut iter = c.iter();
                if let Some(first) = iter.next() {
                    write!(f, "{:?}", first)?;
                    for item in iter {
                        write!(f, " {:?}", item)?;
                    }
                }
                write!(f, " }}")
            },

            other => write!(f, "{}", other) //TODO: Error here?
        }
    }
}

