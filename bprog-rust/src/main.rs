extern crate core;

use std::collections::VecDeque;
use std::io;
use std::io::{BufReader, Read, Write};
use bprog::interpreter::run;
use bprog::numeric::Numeric;
use bprog::op::Op;
use bprog::parsed::Parsed;
use bprog::parsing::{parse, parse_primitives};
use bprog::stack::Stack;
use bprog::types::{Constraint, Type};
use bprog::utility::get_tokens;


fn main() {

    let val = Numeric::Integer(4);
    let val2 = Numeric::Float(0.0);
    let operation = Parsed::Function("+".parse::<Op>().unwrap());
    println!("{}", operation);
    println!("{}", parse_primitives("True").unwrap());
    println!("Int implements Num: {}", Constraint::Num.is_satisfied_by(&Type::Integer));
    let val3 = &val / &val2;
    println!("{}", val3);
    println!("{}", "+".parse::<Op>().unwrap().get_signature());
    println!("{}", (val3 != Numeric::Integer(5)));

    let s = Parsed::String("hello ".to_string());
    let s2 = Parsed::String("world!".to_string());
    let s3 = &s + &Parsed::Num(Numeric::Integer(32));

    println!("{}", &Parsed::Num(Numeric::Integer(0)) - &Parsed::Num(Numeric::Integer(1)));

    /*
    let mut in_file = File::open("./test_program.txt").unwrap();

    let mut program_tokens= vec![];
    match get_tokens(Some(&mut in_file)) {
        Ok(mut tok) => {
            let (mut one, mut two) = parse(tok);
            one.iter().for_each(|t| println!("{} ", &t));
            program_tokens = one;
        }
        Err(_) => {println!("didnt work in main")}
    }

    let mut stack = stack::Stack::new();
*/
    let mut stack: Stack<Parsed> = Stack::new();

    loop {
        print!("bprog > ");
        io::stdout().flush().expect("TODO: panic message");
        match get_tokens(None) {
            Ok(mut tokens) => {
                let (mut stack_tokens, _two) = parse(tokens);
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
