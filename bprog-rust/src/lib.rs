use crate::interpreter::{Binding, run};
use crate::parsed::Parsed;
use crate::parsing::parse;
use crate::stack::Stack;
use crate::utility::to_tokens;
use std::collections::{HashMap, VecDeque};

// integration testing
pub mod stack;
pub mod utility;
pub mod parsing;
pub mod parsed;
pub mod numeric;
pub mod stack_error;
pub mod op;
pub mod interpreter;
pub mod types;

pub fn t(input: &str) -> String {
    // Warning: don't move this function to another module, as integration tests in
    // directory `tests` with `cargo test` will only look into lib.rs, so make your parse and
    // execution functions public and import them here.

    // The following test function should:
    // 1. invoke parser (+lexer) with input string
    // 2. invoke interpreter with tokens from parser as input
    // 3. transform the result to a string (tip: implement Display traits)

    let mut stack: Stack<Parsed> = Stack::new();
    let mut dictionary: HashMap<String, Binding> = HashMap::new();
    let (parsed, _) = parse(to_tokens(&mut input.to_string()));
    run(&mut stack, &mut VecDeque::from(parsed), &mut dictionary);
    format!("{}", stack.top().unwrap())

}