use std::cmp::max;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use crate::stack::Stack;

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
    Error(Error),
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
            (StackToken::Error(_), StackToken::Error(_)) => true,
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
            StackToken::Error(s) => write!(f, "ERROR: {}", s),
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

/// Parses a vector of string tokens into a list of stack tokens and a remainder.
///
/// This function takes a vector of string tokens `tokens` and returns a tuple containing a
/// vector of stack tokens `parsed` and a vector of remaining tokens `remainder`. The `parsed`
/// vector contains the stack tokens created by parsing `tokens`. The `remainder` vector
/// contains any tokens that were not parsed. This function recursively parses tokens until
/// it encounters a closing brace or bracket.
///
/// # Arguments
///
/// * `tokens` - A vector of string tokens to be parsed.
///
/// # Examples
///
/// ```
/// let tokens = vec![
///     "{", "True", "}", "binding", ":", "\"Hello,", "world!\"", "]", "test"
/// ];
///
/// let (parsed, remainder) = parse(tokens);
///
/// assert_eq!(
///     parsed,
///     vec![
///         StackToken::Block(vec![StackToken::Boolean(true)]),
///         StackToken::Binding("binding".to_string()),
///         StackToken::List(vec![StackToken::String("Hello, world!".to_string())])
///     ]
/// );
///
/// assert_eq!(remainder, vec!["test"]);
/// ```
pub(crate) fn parse(mut tokens: Vec<String>) -> (Vec<StackToken>, Vec<String>) {
    let mut parsed: Vec<StackToken> = vec![];
    loop {
        if let Some(t) = tokens.clone().get(0) {
            match t.as_str() {
                "}" | "]" =>  {
                    return (parsed.clone(), tokens[1..].to_vec())
                }
                "{" | "[" => {
                    tokens = tokens[1..].to_vec();
                    let mut content = vec![];
                    (content, tokens) = parse(tokens.clone());
                    parsed.push(if t == "{" { StackToken::Block(content.clone()) } else { StackToken::List(content.clone()) });

                },
                "\"" => {
                    tokens = tokens[1..].to_vec();
                    let result = get_section(&mut tokens, "\"");
                    match result {
                        Some((mut section, mut remainder)) => {
                            tokens = remainder;
                            parsed.push(StackToken::String(section.join(" ")));
                        }
                        None => {println!("didnt work");}
                    }
                },
                "True" | "False" => {
                    parsed.push(StackToken::Boolean(t == "True"));
                    tokens = tokens[1..].to_vec()
                },
                other => {}
            }
        } else {
            break
        }
    }
    return (parsed, tokens.clone())
}

/// Extracts a section of a vector of strings delimited by a specified string.
///
/// This function takes a mutable reference to a vector of strings `tokens` and a string
/// `delimiter`. It returns an `Option` type containing a tuple of two vectors of strings,
/// representing a section of `tokens` and the remaining elements of `tokens` respectively.
/// The section is delimited by the first occurrence of `delimiter` in `tokens`. If `delimiter`
/// is not found in `tokens`, `None` is returned.
///
/// # Examples
///
/// ```
/// let mut tokens = vec!["apple", "banana", "cherry", "date", "elderberry"];
/// let delimiter = "cherry";
/// let section = get_section(&mut tokens, delimiter).unwrap();
/// assert_eq!(section.0, ["apple", "banana"]);
/// assert_eq!(section.1, ["date", "elderberry"]);
/// ```
pub fn get_section (tokens: &mut Vec<String>, delimiter: &str) -> Option<(Vec<String>, Vec<String>)> {
    let idx = tokens
        .iter()
        .position(|t| t.eq(delimiter));
    return match idx {
        Some(pos) => {
            let (section, remainder) = tokens.split_at(pos);
            let mut ret = section.to_vec();
            Some((ret, remainder[1..].to_vec()))
        },
        None => None // TODO: Error
    }
}


/*
pub fn operations_map() -> HashMap<String, Op> {
    let mut binding_list = vec!(
        ("+", Op::Arithmetic(binary_numerical(false, add))),
        ("-", Op::Arithmetic(binary_numerical(false, sub))),
        ("/", Op::Arithmetic(binary_numerical(false, div))),
        ("*", Op::Arithmetic(binary_numerical(false, mul))),
        ("div", Op::Arithmetic(binary_numerical(true, div))),
    );
    let res: Vec<(String, Op)> = binding_list.into_iter().map(|s| (s.0.to_string(), s.1)).collect();
    return HashMap::from_iter(res)
}
*/

fn binary_numerical(strict_type: bool, op: fn(a: f64, b: f64) -> f64) -> Box<dyn Fn(&mut Stack<StackToken>) -> ()> {
    return Box::new(move |stack: &mut Stack<StackToken>| {
        let mut rhs = StackToken::Empty;
        let mut lhs = StackToken::Empty;
        if let Some(token) = stack.pop() {
            rhs = token;
        }
        if let Some(token) = stack.pop() {
            lhs = token;
        }
        if rhs == StackToken::Empty || lhs == StackToken::Empty {
            stack.push(StackToken::Error(fmt::Error));
            return
        }
        // If strict_type is on, implicit type conversion is off.
        if strict_type && ( lhs != rhs ) {
            stack.push(StackToken::Error(fmt::Error));
            return
        }

        match (lhs, rhs){
            (StackToken::Integer(l), StackToken::Integer(r)) => {
                stack.push(StackToken::Integer(op(l as f64, r as f64) as i32))
            },
            (StackToken::Float(l), StackToken::Integer(r)) => {
                stack.push(StackToken::Float(op(l, r as f64)))
            },
            (StackToken::Integer(l), StackToken::Float(r)) => {
                stack.push(StackToken::Float(op(l as f64, r)))
            },
            (StackToken::Float(l), StackToken::Float(r)) => {
                stack.push(StackToken::Float(op(l, r)))
            },
            (_, _) => {
                stack.push(StackToken::Error(fmt::Error)) // TODO: error handling
            }
        }
    })
}

fn add(a: f64, b: f64) -> f64 {
    return a + b
}

fn sub(a: f64, b: f64) -> f64{
    return a - b
}

fn mul(a: f64, b: f64) -> f64{
    return a * b
}

fn div(a: f64, b: f64) -> f64 {
    return a / b
}
































