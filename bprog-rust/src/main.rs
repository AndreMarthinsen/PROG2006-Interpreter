extern crate core;

use std::collections::VecDeque;
use std::io;
use std::io::Write;
use bprog::interpreter::run;
use bprog::parsed::Parsed;
use bprog::parsing::parse;
use bprog::stack::Stack;
use bprog::utility::get_tokens;


fn main() {

    let mut stack: Stack<Parsed> = Stack::new();

    loop {
        print!("bprog > ");
        io::stdout().flush().expect("TODO: panic message");
        match get_tokens(None) {
            Ok( tokens) => {
                let (stack_tokens, _two) = parse(tokens);
                let mut run_tokens = VecDeque::from(stack_tokens);
                run(&mut stack, &mut run_tokens);
                //match stack_tokens {
                //    None => {println!("something went to shits");}
                //    Some(ts) => println!("{:?}", &ts)
                //}
                stack.display_all_contents();
            }
            _ => {}
        }
        println!();
    };

}
