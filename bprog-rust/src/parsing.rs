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
///
pub fn parse(tokens: &mut VecDeque<String>) -> Vec<Parsed> {
    let mut parsed: Vec<Parsed> = vec![];

    while let Some(t) = tokens.pop_front() {
        if let Some(p) = parse_primitives(t.as_str()) {
            parsed.push(p);
            continue;
        };
        if let Some(p) = parse_operations(t.as_str()) {
            parsed.push(p);
            continue;
        };
        match t.as_str() {
            "}" | "]" =>  {
                return parsed;
            },
            "{" | "[" => {
                let content: Vec<Parsed>;
                content = parse(tokens);
                parsed.push(if t == "{" {
                    Parsed::Quotation(VecDeque::from(content.clone())) }
                else {
                    Parsed::List(content.clone())
                });
            },
            "\"" => {
                let result = get_section(tokens, "\"");
                match result {
                    Some(section) => {
                        parsed.push(Parsed::String(section.join(" ")));
                    }
                    None => {println!("didnt work");}
                };
            },
            other => {
                parsed.push(Parsed::Symbol(other.to_string()));
            }
        };
    }{};
    return parsed;
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
pub fn get_section (tokens: &mut VecDeque<String>, delimiter: &str) -> Option<Vec<String>> {
    let mut section = Vec::new();
    while let Some(t) = tokens.pop_front() {
        if t.eq(delimiter) {
            return Some(section)
        } else {
            section.push(t.clone())
        }
    }{}
    None
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
    let parsed = parse(&mut VecDeque::from(to_tokens(&mut string.to_string())));
    Parsed::Quotation(VecDeque::from(parsed))
}


























