
/////////////////////////// OP ////////////////////////////////////////////////////////////////////

use std::{fmt, io};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::str::FromStr;
use crate::numeric::Numeric;
use crate::parsed::Parsed;
use crate::stack_error::StackError;
use crate::types::{Params, Constraint, heterogeneous_binary, homogenous_binary, nullary, Signature, Type, TypeClass, unary};
use crate::types;


#[derive(Clone)]
/// enumerator of operations, i.e. specific functions.
pub enum Op {
    Void,
    IOPrint,
    IORead,
    ParseInt,
    ParseFloat,
    ParseWords,
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    LT,
    GT,
    EQ,
    And,
    Or,
    Not,
    ListHead,
    ListTail,
    ListEmpty,
    ListLength,
    ListCons,
    ListAppend,
    Each,
    Map,
    Foldl,
    If,
    Loop,
    Times,
    Exec,
    Assign,
    AssignFunc,
    AsSymbol,
    EvalSymbol,
    Dup,
    Swap,
    Pop,
}



impl Op {

    pub fn exec_nullary(&self) -> Parsed {
        match self {
            Op::IORead => {
                print!("input: ");
                io::stdout().flush().unwrap();
                let mut string = String::new();
                if let Ok(_) = io::stdin().read_line(&mut string) {
                    string.pop();
                    Parsed::String(string)
                } else {
                    Parsed::Error(StackError::InvalidRight)
                }
            }
            _ => Parsed::Error(StackError::InvalidRight)
        }
    }

    pub fn exec_unary(&self, arg: Parsed) -> Parsed {
        match self {
            Op::IOPrint => {
                println!("output: {}", arg);
                Parsed::Void
            },
            Op::ListEmpty => {
                Parsed::Bool(arg.size() == Parsed::Num(Numeric::Integer(0)))
            },
            Op::ListLength =>  {
                arg.size()
            },
            Op::ListHead => {
                match arg {
                    Parsed::List(v) => {
                        if let Some(val) = v.get(0){
                            val.clone()
                        } else {
                            Parsed::Error(StackError::InvalidRight)
                        }
                    },
                    _ => panic!("head not supported for {}", arg)
                }
            },
            Op::ListTail => {
                match arg {
                    Parsed::List(v) => {
                        if !v.is_empty() {
                            Parsed::List(v[1..].to_vec())
                        } else {
                            Parsed::Error(StackError::InternalBug)
                        }
                    },
                    _ => panic!("tail not support")
                }
            }
            Op::Pop => {
                Parsed::Void
            },
            Op::Dup => {
                Parsed::Quotation(VecDeque::from(vec![arg.clone(), arg.clone()]))
            },
            Op::Exec => {
                return arg
            }
            _ => Parsed::Error(StackError::InvalidBoth)
        }
    }

    pub fn exec_binary(&self, lhs: &Parsed, rhs: &Parsed) -> Parsed {
        match self {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
            Op::IntDiv => lhs / rhs,
            Op::GT => Parsed::Bool(lhs > rhs),
            Op::LT => Parsed::Bool(lhs < rhs),
            Op::EQ => Parsed::Bool(lhs == rhs),
            Op::And => lhs & rhs,
            Op::Or => lhs | rhs,
            Op::ListAppend => lhs + rhs,
            Op::ListCons => {
                &Parsed::List(vec![lhs.clone()]) + rhs
            },
            Op::Swap => {
                Parsed::Quotation(VecDeque::from(vec![rhs.clone(), lhs.clone()]))
            },
            _ => Parsed::Error(StackError::InvalidBoth)
        }
    }

    pub fn exec_temary(&self) {

    }


    pub fn get_signature(&self) -> Signature {
        match self {
            Op::Void => nullary(Constraint::Void),
            Op::IOPrint =>
                unary(Constraint::Display, Constraint::Void),
            Op::IORead =>
                nullary(Constraint::String),
            Op::ParseInt =>
                unary(Constraint::String, Constraint::Integer),
            Op::ParseFloat =>
                unary(Constraint::String, Constraint::Float),
            Op::ParseWords =>
                unary(Constraint::String, Constraint::List),
            Op::Add | Op::Sub | Op::Mul | Op::Div =>
                homogenous_binary(Constraint::Num, Constraint::Num),
            Op::IntDiv =>
                homogenous_binary(Constraint::Integer, Constraint::Integer),
            Op::LT | Op::GT | Op::EQ =>
                homogenous_binary(Constraint::Ord, Constraint::Bool),
            Op::And | Op::Or =>
                homogenous_binary(Constraint::Boolean, Constraint::Bool),
            Op::Not =>
                unary(Constraint::Num, Constraint::Num),
            Op::ListHead =>
                unary(Constraint::List, Constraint::Any),
            Op::ListTail =>
                unary(Constraint::List, Constraint::List),
            Op::ListEmpty => {
                unary(Constraint::List, Constraint::Bool)
            },
            Op::ListLength => {
                unary(Constraint::List, Constraint::Integer)
            },
            Op::ListCons => {
                heterogeneous_binary(
                    Constraint::Any,
                    Constraint::List,
                    Constraint::List
                )
            },
            Op::ListAppend => {
                homogenous_binary(Constraint::List, Constraint::List)
            }
            Op::Each => { //TODO: modifying arguments? quotations expected from tree.
                let mut sig = unary(Constraint::List, Constraint::Void);
                sig.modifers = Params::Unary(
                    Constraint::Function(
                        Box::new(unary(Constraint::Any, Constraint::Void))
                    )
                );
                sig
            },
            Op::Map => {
                unary(Constraint::List, Constraint::List)
            },
            Op::Foldl => {
                heterogeneous_binary(
                    Constraint::Any,
                    Constraint::List,
                    Constraint::Any
                )
            }
            Op::If =>  {
                unary(Constraint::Boolean, Constraint::Any)
            },
            Op::Loop => { // TODO: Very unclear how this one should work
                nullary(Constraint::Void)
            },
            Op::Times => { // TODO: Ditto
                unary(Constraint::Integer, Constraint::Any)
            }
            Op::Exec => {
                unary(Constraint::Quotation, Constraint::Any)
            }
            Op::Assign => {
                heterogeneous_binary(
                    Constraint::Symbol,
                    Constraint::Any,
                    Constraint::Void
                )
            },
            Op::AssignFunc => {
                heterogeneous_binary(
                    Constraint::Symbol,
                    Constraint::Quotation,
                    Constraint::Void
                )
            }
            Op::AsSymbol => {
                nullary(Constraint::Symbol)
            }
            Op::EvalSymbol => {
                unary(Constraint::Symbol, Constraint::Any)
            },
            Op::Dup => {
                unary(Constraint::Any, Constraint::Any)
            },
            Op::Swap => {
                heterogeneous_binary(
                    Constraint::Any,
                    Constraint::Any,
                    Constraint::Any
                )
            }
            Op::Pop => {
                unary(Constraint::Any, Constraint::Void)
            }
        }
    }
}



/// Display for Operations
impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Op::Void => write!(f, "()"),
            Op::IOPrint => write!(f, "print"),
            Op::IORead => write!(f, "read"),
            Op::ParseInt => write!(f, "parseInteger"),
            Op::ParseFloat => write!(f, "parseFloat"),
            Op::ParseWords => write!(f, "parseWords"),
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
            Op::IntDiv => write!(f, "div"),
            Op::LT => write!(f, "<"),
            Op::GT => write!(f, ">"),
            Op::EQ => write!(f, "=="),
            Op::And => write!(f, "&&"),
            Op::Or => write!(f, "||"),
            Op::Not => write!(f, "not"),
            Op::ListHead => write!(f, "head"),
            Op::ListTail => write!(f, "tail"),
            Op::ListEmpty => write!(f, "empty"),
            Op::ListLength => write!(f, "length"),
            Op::ListCons => write!(f, "cons"),
            Op::ListAppend => write!(f, "append"),
            Op::Each => write!(f, "each"),
            Op::Map => write!(f, "map"),
            Op::Foldl => write!(f, "foldl"),
            Op::If => write!(f, "if"),
            Op::Loop => write!(f, "loop"),
            Op::Times => write!(f, "times"),
            Op::Exec => write!(f, "exec"),
            Op::Assign => write!(f, ":="),
            Op::AssignFunc => write!(f, "fun"),
            Op::AsSymbol => write!(f, "'"),
            Op::EvalSymbol => write!(f, "eval"),
            Op::Dup => write!(f, "dup"),
            Op::Swap => write!(f, "swap"),
            Op::Pop => write!(f, "swap"),
        }
    }
}

/// implements FromStr for Op, allowing the use of .parse() to get Op directly
/// from a string.
impl FromStr for Op {
    type Err = String;  // TODO: StackError? Other?

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "print" => Ok(Op::IOPrint),
            "read" => Ok(Op::IORead),
            "parseInteger" => Ok(Op::ParseInt),
            "parseFloat" => Ok(Op::ParseFloat),
            "parseWords" => Ok(Op::ParseWords),
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            "div" => Ok(Op::IntDiv),
            "<" => Ok(Op::LT),
            ">" => Ok(Op::GT),
            "==" => Ok(Op::EQ),
            "&&" => Ok(Op::And),
            "||" => Ok(Op::Or),
            "not" => Ok(Op::Not),
            "head" => Ok(Op::ListHead),
            "tail" => Ok(Op::ListTail),
            "empty" => Ok(Op::ListEmpty),
            "length" => Ok(Op::ListLength),
            "cons" => Ok(Op::ListCons),
            "append" => Ok(Op::ListAppend),
            "each" => Ok(Op::Each),
            "map" => Ok(Op::Map),
            "foldl" => Ok(Op::Foldl),
            "if" => Ok(Op::If),
            "loop" => Ok(Op::Loop),
            "times" => Ok(Op::Times),
            "exec" => Ok(Op::Exec),
            ":=" => Ok(Op::Assign),
            "fun" => Ok(Op::AssignFunc),
            "'" => Ok(Op::AsSymbol),
            "eval" => Ok(Op::EvalSymbol),
            "pop" => Ok(Op::Pop),
            "swap" => Ok(Op::Swap),
            "dup" => Ok(Op::Dup),
            _ => Err(format!("unknown operation: {}", s)),
        }
    }
}

