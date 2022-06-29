use std::{
    env, fs,
    io::{self, Write},
    process,
};

pub mod lexer;
use cluster::*;
use lexer::*;
use token::*;

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
    let mut stack: Vec<i32> = Vec::new();

    //Value used to build a number before it's pushed to the stack
    let mut pre_push = 0;

    let mut to_print = String::new();
    let mut input = String::new();

    //Index into vec
    //I need to jump back and forth in the vec so I can't just iter
    let mut x = 0;

    while x < lexed_code.len() {
        for value in &lexed_code[x].top {
            match *value {
                //0 - F value
                TopSet::Number(a) => {
                    pre_push = (pre_push << 4) + a;
                }
                //Push to stack
                TopSet::Push => {
                    stack.push(pre_push);
                    pre_push = 0;
                }
                //Pop top off stack
                TopSet::Pop => {
                    stack.pop();
                }
            }
        }
        for value in &lexed_code[x].bottom {
            match *value {
                //Pop the top of the stack
                //If 0, skip the next cluster
                BottomSet::If => {
                    let if_check = pop_stack(&mut stack)?;

                    if if_check == 0 {
                        x += 1;
                    }
                }
                BottomSet::Print => {
                    //Pop top of stack and print
                    let print_char = pop_stack(&mut stack)?;

                    if print_char < 0 {
                        return Err("Invalid char value!");
                    }

                    let print_char = match char::from_u32(print_char as u32) {
                        Some(x) => x,
                        None => {
                            return Err("Invalid char value!");
                        }
                    };

                    to_print = format!("{}{}", to_print, print_char);
                }
                BottomSet::Input => {
                    if input.is_empty() {
                        print!("{}", to_print);
                        io::stdout().flush().expect("Failed to flush buffer");

                        to_print = "".to_string();

                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");

                        //The input will have a newline at the end, so it's removed
                        //A null byte is added to the end so that it is possible to tell when the
                        //string ends
                        input.pop();
                        input = format!("{}{}", input, "\0")
                    }

                    let mut input_chars = input.chars();
                    //This is ok as the previous if statement
                    //makes sure the string has something in it
                    stack.push(input_chars.next().unwrap() as i32);
                    input = input_chars.collect();
                }
                BottomSet::Dup => {
                    //Duplicate top value of stack
                    let single = pop_stack(&mut stack)?;

                    stack.push(single);
                    stack.push(single);
                }
                BottomSet::Add => {
                    let a = pop_stack(&mut stack)?;
                    let b = pop_stack(&mut stack)?;

                    stack.push(a + b);
                }
                BottomSet::Sub => {
                    let a = pop_stack(&mut stack)?;
                    let b = pop_stack(&mut stack)?;

                    stack.push(b - a);
                }
                BottomSet::Mul => {
                    let a = pop_stack(&mut stack)?;
                    let b = pop_stack(&mut stack)?;

                    stack.push(a * b);
                }
                BottomSet::Div => {
                    let a = pop_stack(&mut stack)?;
                    let b = pop_stack(&mut stack)?;

                    stack.push(b / a);
                }
                BottomSet::Mod => {
                    let a = pop_stack(&mut stack)?;
                    let b = pop_stack(&mut stack)?;

                    stack.push(b % a);
                }
            }
        }

        x += 1;
    }

    println!("{}", to_print);

    Ok(())
}

fn pop_stack(stack: &mut Vec<i32>) -> Result<i32, &'static str> {
    match stack.pop() {
        Some(x) => Ok(x),
        None => Err("Out of stack values!"),
    }
}
