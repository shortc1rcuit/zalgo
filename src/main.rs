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

    //Index into vec
    //I need to jump back and forth in the vec so I can't just iter
    let mut x = 0;

    while x < lexed_code.len() {
        for value in &lexed_code[x].top {
            if *value <= 0xF {
                //0-F value
                pre_push = (pre_push << 4) + value;
            } else if *value == 0x10 {
                //Push to stack
                stack.push(pre_push);
                pre_push = 0;
            } else if *value == 0x11 {
                //Pop top of stack
                stack.pop();
            }
        }
        for value in &lexed_code[x].bottom {
            //Pop the top of the stack
            //If 0, skip the next cluster
            if *value == 0x1C {
                let if_check = pop_stack(&mut stack)?;

                if if_check == 0 {
                    x += 1;
                }
            } else if *value == 0x1D {
                //Pop top of stack and print
                let print_char = pop_stack(&mut stack)?;

                let print_char = match char::from_u32(print_char) {
                    Some(x) => x,
                    None => {
                        return Err("Invalid char value!");
                    }
                };

                result = format!("{}{}", result, print_char);
            } else if *value == 0x48 {
                let single = pop_stack(&mut stack)?;

                stack.push(single);
                stack.push(single);
            }
        }

        x += 1;
    }

    println!("{}", result);

    Ok(())
}

fn pop_stack(stack: &mut Vec<u32>) -> Result<u32, &'static str> {
    match stack.pop() {
        Some(x) => Ok(x),
        None => {
            return Err("Out of stack values!");
        }
    }
}
