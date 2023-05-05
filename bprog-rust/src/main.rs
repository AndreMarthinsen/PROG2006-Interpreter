extern crate core;

use std::fmt::Error;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use crate::parsing::{get_section, parse};
use crate::token::{Numeric, Op, StackError, Parsed};
use crate::utility::get_tokens;

mod stack;
mod utility;
mod parsing;
mod token;

fn main() {

    let val = Numeric::Int32(4);
    let val2 = Numeric::Float64(0.0);
    println!("{}", val2);
    let val3 = &val / &val2;
    println!("{}", val3);

    println!("{}", (val3 != Numeric::Int32(5)));

    let s = Parsed::String("hello ".to_string());
    let s2 = Parsed::String("world!".to_string());
    let s3 = &s + &Parsed::Num(Numeric::Int32(32));

    println!("{}", &Parsed::Num(Numeric::Int32(0)) - &Parsed::Num(Numeric::Int32(1)));

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


    loop {
        match get_tokens(None) {
            Ok(mut tokens) => {
                let (mut stack_tokens, mut two) = parse(tokens);
                println!("{:?}", &stack_tokens);
                stack_tokens.iter()
                    .for_each(|t| {
                        match t {
                            StackToken::Operation(op) => {
                                match op {
                                    Op::Void => {}
                                }
                            }
                            others => {
                                stack.push(others.clone())
                            }
                        }
                    })
                }
                //match stack_tokens {
                //    None => {println!("something went to shits");}
                //    Some(ts) => println!("{:?}", &ts)
                //}
            _ => {}
        }
        stack.display_all_contents();
    };
*/
}
