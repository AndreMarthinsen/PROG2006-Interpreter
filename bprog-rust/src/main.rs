extern crate core;

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::Write;
use bprog::interpreter::{Binding, run};
use bprog::parsed::Parsed;
use bprog::parsing::{parse};
use bprog::stack::Stack;
use bprog::utility::{get_input, to_tokens};


fn main() {

    let mut stack: Stack<Parsed> = Stack::new();
    let mut dictionary: HashMap<String, Binding> = HashMap::new();

    let mut prelude = File::open("prelude.bprog").unwrap();
    if let Ok(mut pre_definitions) = get_input(Some(&mut prelude)) {
        let mut run_tokens = VecDeque::from(parse(&mut to_tokens(&mut pre_definitions)));
        run(&mut stack, &mut run_tokens, &mut dictionary);
        println!("prelude definitions loaded!");
    }

    loop {
        print!("bprog > ");
        io::stdout().flush().expect("TODO: panic message");
        match get_input(None) {
            Ok(mut tokens) => {
                let stack_tokens = parse(&mut to_tokens(&mut tokens));
                let mut run_tokens = VecDeque::from(stack_tokens);
                run(&mut stack, &mut run_tokens, &mut dictionary );
                println!("stack > {}", stack.contents_to_string());
            }
            _ => {}
        }
        println!();
    };

}
