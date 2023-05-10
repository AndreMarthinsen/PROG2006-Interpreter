extern crate core;

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::{env, io};
use std::io::Write;
use bprog::interpreter::{Binding, run};
use bprog::parsed::Parsed;
use bprog::parsing::{parse};
use bprog::stack::Stack;
use bprog::utility::{get_input, to_tokens};

fn print_help() {
    println!(
        "Arguments:\n\
        \t-r,  --repl-mode      Runs bprog in repl mode, allowing repeated input and print\n\
        \t                      of stack post input execution.\n\
        \t--src=\"<filename\">    Runs the contents of the specified file.\n\
        \t-h,  --help           Provides information about program arguments and use cases.\n\
        \t-i,  --info           Provides extended information about REPL mode usage.\n\
        \t-dbg,--debug          Prints all program tokens before executing the program.\n"
    )
}

fn print_token_debug(token: &Parsed, depth: usize) {
    match token {
        Parsed::Quotation(contents) => {
            let tab = "\t".repeat(depth);
            println!("\t{}Type:  \x1b[33mQuotation\x1b[0m \n\t{}Value:\n \n\t\t{}{{", tab, tab, tab);
            contents.iter().for_each(|t| print_token_debug(t, depth + 2));
            println!("\t\t{}}}\n", "\t".repeat(depth));
        },
        _ => println!("\t{}Type:  \x1b[33m{}\x1b[0m \n\t{}Value: \x1b[33m{}\x1b[0m\n"
                 ,"\t".repeat(depth), token.get_type(), "\t".repeat(depth), token)
    }
}


fn main() {
    let mut use_repl_mode = false;
    let mut use_normal_mode = false;
    let mut source_file = String::new();
    let mut use_help = false;
    let mut debug = false;

    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    let mut terminate_early = args.len() == 0;
    args.iter().for_each(| arg | {
        if arg.starts_with("--src=") {
            source_file = arg.trim_start_matches("--src=").to_string();
            use_normal_mode = true;
        } else {
            match arg.as_str() {
                "-r" | "--repl-mode" => {
                    use_repl_mode = true;
                },
                "-h" | "--help" => {
                    use_help = true;
                    terminate_early = true;
                },
                "-dbg" | "--debug" =>  {
                    debug = true;
                }
                _ => {
                    println!("Unrecognized arg <\x1b[31m{}\x1b[0m>.", arg);
                    terminate_early = true;
                }
            };
        }
    } );
    terminate_early = terminate_early || !(use_repl_mode ^ use_normal_mode);
    match (use_repl_mode, use_normal_mode, use_help) {
        (_,_, true) => {
            print_help()
        },
        (true, true, _) => {
            println!("Can only run either source file or repl mode. Use --help for more information");
        },
        (false, false, false) => {
            println!("No valid arguments provided. Use --help for more information");
        },
        _ => {}
    }
    if terminate_early { return }





    let mut stack: Stack<Parsed> = Stack::new();
    let mut dictionary: HashMap<String, Binding> = HashMap::new();

    let mut prelude = File::open("./prelude.bprog").unwrap();
    if let Ok(mut pre_definitions) = get_input(Some(&mut prelude)) {
        let mut run_tokens = VecDeque::from(parse(&mut to_tokens(&mut pre_definitions)));
        run(&mut stack, &mut run_tokens, &mut dictionary, true);
        println!("prelude definitions loaded!");
    }

    if use_repl_mode {
        'repl: loop {
            print!("bprog > ");
            io::stdout().flush().expect("TODO: panic message");
            match get_input(None) {
                Ok(mut input_string) => {
                    let mut tokens = to_tokens(&mut input_string);
                    if let Some(first_element) = tokens.front() {
                        if let Some(_) = match first_element.as_str() {
                            ":dbg" => {
                                debug = !debug;
                                None
                            },
                            ":i" => {
                                if let Some(parsed) = stack.top() {
                                    print_token_debug(parsed, 1);
                                }
                                None
                            },
                            ":h" => {
                                println!("repl operations:\n\
                                \t:dbg - Toggles debug mode, showing details about every input token.\n\
                                \t:i   - Shows type and value of the top stack value.\n\
                                \t:h   - Shows repl operations.\n\
                                \t:c   - Clears the stack of contents.\n\
                                \t:q   - Ends REPL mode and exits bprog application.\n");
                                None
                            },
                            ":q" => {
                                println!("Exiting bprog");
                                break 'repl;
                            },
                            ":c" => {
                                println!("\tEmptying stack...");
                                stack.clear();
                                None
                            }
                            _ => Some(1)
                        } {} else {
                            tokens.clear();
                        }
                    }
                    let parsed_tokens = parse(&mut tokens);
                    if debug { parsed_tokens.iter().for_each(|t| print_token_debug(t, 0) )}

                    let mut run_tokens = VecDeque::from(parsed_tokens);
                    run(&mut stack, &mut run_tokens, &mut dictionary , false);
                    println!("stack > {}", stack.contents_to_string());
                }
                _ => {}
            }
            println!();
        };
    } else {
        if let Ok(mut program_file) = File::open(&source_file) {
            match get_input(Some(&mut program_file)) {
                Ok(mut tokens) => {
                    let stack_tokens = parse(&mut to_tokens(&mut tokens));
                    if debug {
                        println!("\ndebug mode: Displaying parsed input");
                        stack_tokens.iter()
                            .for_each( |t| print_token_debug(t, 0));
                        println!()
                    }
                    let mut run_tokens = VecDeque::from(stack_tokens);
                    println!("running...");
                    run(&mut stack, &mut run_tokens, &mut dictionary, true );
                    if stack.size() != 1 {
                        println!("stack: {}", stack.contents_to_string());
                        panic!("Program did not evaluate to a single value!")
                    } else {
                        println!("final stack value: {}", stack.top()
                            .expect("guaranteed to contain one value."))
                    }
                }
                _ => {}
            }
        } else {
            println!("no file with path \"{}\" found.", source_file);
        }
    }
}
