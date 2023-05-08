extern crate core;

use std::collections::{HashMap, VecDeque};
use std::io;
use std::io::Write;
use bprog::interpreter::{Binding, run};
use bprog::parsed::Parsed;
use bprog::parsing::parse;
use bprog::stack::Stack;
use bprog::utility::{get_input, to_tokens};


fn main() {

    let mut stack: Stack<Parsed> = Stack::new();
    let mut dictionary: HashMap<String, Binding> = HashMap::new();
    loop {
        print!("bprog > ");
        io::stdout().flush().expect("TODO: panic message");
        match get_input(None) {
            Ok(mut tokens) => {
                let stack_tokens = parse(&mut VecDeque::from(to_tokens(&mut tokens)));
                let mut run_tokens = VecDeque::from(stack_tokens);
                run(&mut stack, &mut run_tokens, &mut dictionary );
                //match stack_tokens {
                //    None => {println!("something went to shits");}
                //    Some(ts) => println!("{:?}", &ts)
                //}
                print!("stack > ");
                stack.display_all_contents();
            }
            _ => {}
        }
        println!();
    };

}
