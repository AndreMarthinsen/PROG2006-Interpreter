
/////////////////////////// OP ////////////////////////////////////////////////////////////////////

use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone)]
/// enumerator of operations, i.e. specific functions.
pub enum Op {
    //  Void,
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
    EvalSymbol
}




/// Display for Operations
impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
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
            _ => Err(format!("unknown operation: {}", s)),
        }
    }
}

