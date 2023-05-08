use std::collections::{VecDeque};
use crate::numeric::Numeric;
use crate::parsed::Parsed;
use crate::op::Op;
use crate::utility::to_tokens;


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
pub fn parse(mut tokens: Vec<String>) -> (Vec<Parsed>, Vec<String>) {
    let mut parsed: Vec<Parsed> = vec![];
    loop {
        if let Some(t) = tokens.clone().get(0) {
            if let Some(p) = parse_primitives(t) {
                parsed.push(p);
                tokens = tokens[1..].to_vec();
                continue;
            }
            if let Some(p) = parse_operations(t) {
                parsed.push(p);
                tokens = tokens[1..].to_vec();
                continue;
            }
            match t.as_str() {
                "}" | "]" =>  {
                    return (parsed.clone(), tokens[1..].to_vec())
                }
                "{" | "[" => {
                    tokens = tokens[1..].to_vec();
                    let mut content = Vec::new();
                    (content, tokens) = parse(tokens.clone());
                    parsed.push(if t == "{" {
                        Parsed::Quotation(VecDeque::from(content.clone())) }
                    else {
                        Parsed::List(content.clone())
                    });

                },
                "\"" => {
                    tokens = tokens[1..].to_vec();
                    let result = get_section(&mut tokens, "\"");
                    match result {
                        Some((section, remainder)) => {
                            tokens = remainder;
                            parsed.push(Parsed::String(section.join(" ")));
                        }
                        None => {println!("didnt work");}
                    }
                },
                "True" | "False" => {
                    parsed.push(Parsed::Bool(t == "True"));
                    tokens = tokens[1..].to_vec()
                },
                other => {
                    parsed.push(Parsed::Symbol(other.to_string()));
                    tokens = tokens[1..].to_vec()
                }
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
/// ```
pub fn get_section (tokens: &mut Vec<String>, delimiter: &str) -> Option<(Vec<String>, Vec<String>)> {
    let idx = tokens
        .iter()
        .position(|t| t.eq(delimiter));
    return match idx {
        Some(pos) => {
            let (section, remainder) = tokens.split_at(pos);
            let ret = section.to_vec();
            Some((ret, remainder[1..].to_vec()))
        },
        None => None // TODO: Error
    }
}


/// Parses Integer, Float and Boolean from a string.
pub fn parse_primitives(token: & str) -> Option<Parsed> {
    if let Ok(val) = token.parse::<Numeric>() {
        return Some(Parsed::Num(val));
    }
    if token == "True" {
        return Some(Parsed::Bool(true))
    }
    if token == "False" {
        return Some(Parsed::Bool(false))
    }
    if let Ok(val) = token.parse::<f64>() {
        return Some(Parsed::Num(Numeric::Float(val)));
    }
    return None;
}


pub fn parse_operations(token: & str) -> Option<Parsed> {
    if let Ok(op) = token.parse::<Op>() {
        return Some(Parsed::Function(op))
    }
    None
}

/// Function for use with tests.
pub fn parse_to_quotation(string: String) -> Parsed {
    let (parsed, _) = parse(to_tokens(&mut string.to_string()));
    Parsed::Quotation(VecDeque::from(parsed))
}


























