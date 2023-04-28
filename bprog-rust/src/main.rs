use std::fmt::Error;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use crate::utility::get_tokens;

mod stack;
mod utility;


fn main() {
    let mut in_file = File::open("./test_program.txt").unwrap();
    let mut stack = stack::Stack::new();


    (0..10).map(|x| x.to_string())
        .for_each(|x| stack.push(x));
    println!("Stack size is {}", stack.size());
    stack.pop().unwrap();
    println!("Stack size is {}", stack.size());
    stack.display_all_contents();
    stack.push(String::from("5"));
    assert_eq!("5", *stack.top().unwrap());

    println!("testing from file");
    match get_tokens(Some(&mut in_file)) {
        Ok(tokens) => {
            println!("ok");
            println!("{:?}", &tokens);
        }
        Err(_) => {}
    }

    loop {
        println!("ok");
        match get_tokens(None) {
            Ok(tokens) => {
                println!("ok");
                println!("{:?}", &tokens);
            }
            Err(_) => break
        }
    };
    println!("program exit due to error in parsing")
}
