use std::{env, fs, process};

pub mod lexer;
use cluster::*;
use lexer::*;

fn main() {
    let filename = get_filename(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let base_code = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        process::exit(1);
    });

    let lexed_code = match lex_text(base_code) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Compile Error: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = interpret_code(lexed_code) {
        eprintln!("Runtime error: {}", e);
        process::exit(1);
    }
}

fn get_filename(mut args: env::Args) -> Result<String, &'static str> {
    args.next();

    let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get filename"),
    };

    Ok(filename)
}

fn interpret_code(lexed_code: Vec<Cluster>) -> Result<(), &'static str> {
    let mut stack: Vec<u32> = Vec::new();

    //Value used to build a number before it's pushed to the stack
    let mut pre_push = 0;

    let mut result = String::new();

    for cluster in lexed_code {
        for value in cluster.top {
            if value <= 0xF {
                //0-F value
                pre_push = (pre_push << 4) + value;
            } else if value == 0x10 {
                //Push to stack
                stack.push(pre_push);
                pre_push = 0;
            }
        }
        for value in cluster.bottom {
            if value == 0x1D {
                //Pop top of stack and print
                let print_char = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return Err("Out of stack values!");
                    }
                };

                let print_char = match char::from_u32(print_char) {
                    Some(x) => x,
                    None => {
                        return Err("Invalid char value!");
                    }
                };

                result = format!("{}{}", result, print_char);
            }
        }
    }

    println!("{}", result);

    Ok(())
}
