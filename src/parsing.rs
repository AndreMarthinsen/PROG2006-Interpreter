use std::collections::{VecDeque};
use crate::numeric::Numeric;
use crate::parsed::Parsed;
use crate::op::Op;
use crate::utility::to_tokens;


/// Parses string tokens into the Parsed enum type, capable of representing
/// a predefined set of types and functions, such as +, -, float and integer.
///
/// # Arguments
///
/// * `tokens` - A vector of string tokens to be parsed.
///
/// # Examples
///
/// ```
/// use std::collections::VecDeque;
/// use bprog::parsing::{parse, parse_to_quotation};
/// use bprog::utility::to_tokens;
///
/// let mut  tokens = VecDeque::from(to_tokens("{ 1 + }"));
/// let expected = parse_to_quotation("1 +".to_string());
///
/// assert_eq!(expected, parse(&mut tokens).pop().unwrap())
///
/// ```
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
                    None => {panic!("\x1b[31merr: failed to find terminating \" token for string \
                    while parsing input.\x1b[0m")}
                };
            },
            other => {
                parsed.push(Parsed::Symbol(other.to_string()));
            }
        };
    }{};
    return parsed;
}

/// Extracts a section of a VecDeque<String> container, stopping when finding
/// the delimiting string. Not finding the delimiter in the container body is
/// considered a failure.
///
/// # Arguments
///
/// `tokens` - container section is removed from.
///
/// `delimiter` - Stop condition. The matching string is removed from `tokens`.
///
/// # Examples
///
/// ```
/// use std::collections::VecDeque;
/// use bprog::parsing::get_section;
/// use bprog::utility::string_vec_deque;
///
/// let mut  tokens = string_vec_deque(&["this", "\"", "remainder"]);
/// let expected = vec!["this".to_string()];
/// assert_eq!(expected, get_section(&mut tokens, "\"").unwrap());
///
/// let mut  tokens = string_vec_deque(&["this", "remainder"]);
/// assert_eq!(None, get_section(&mut tokens, "\""))
///
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
///
/// # Examples
///
/// ```
/// use bprog::numeric::Numeric;
/// use bprog::parsed::Parsed;
/// use bprog::parsing::parse_primitives;
///
/// let test = parse_primitives("1.013ui");
///
/// assert_eq!(None, test);
///
/// let expected = Parsed::Bool(true);
/// let test = parse_primitives("True").unwrap();
///
/// assert_eq!(expected, test)
///
/// ```
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

/// Parses Parsed::Function from &str. Relies on Parsed implementation of FromStr.
///
/// # Examples
///
/// ```
/// use bprog::op::Op;
/// use bprog::parsed::Parsed;
/// use bprog::parsing::parse_operations;
///
/// let expected = Parsed::Function(Op::Add);
/// let test = parse_operations("+").unwrap();
///
/// assert_eq!(expected, test);
///
/// ```
pub fn parse_operations(token: & str) -> Option<Parsed> {
    if let Ok(op) = token.parse::<Op>() {
        return Some(Parsed::Function(op))
    }
    None
}

/// Parses a string into a quotation.
///
/// # Examples
///
/// ```
/// use std::collections::VecDeque;
/// use bprog::parsed::Parsed;
/// use bprog::parsing::parse_to_quotation;
///
/// let expected = Parsed::Quotation(VecDeque::from(vec![Parsed::Bool(true)]));
/// let test = parse_to_quotation("True".to_string());
///
/// assert_eq!(expected, test)
/// ```
pub fn parse_to_quotation(string: String) -> Parsed {
    let parsed = parse(&mut VecDeque::from(to_tokens(&mut string.to_string())));
    Parsed::Quotation(VecDeque::from(parsed))
}


























