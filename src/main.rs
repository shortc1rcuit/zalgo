use std::{env, fs, process};

pub mod lexer;
use cluster::*;
use lexer::*;

pub mod interpreter;
use interpreter::*;

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
    let mut negation = 1;

    let mut input = String::new();

    //Index into vec
    //I need to jump back and forth in the vec so I can't just iter
    let mut x: i32 = 0;

    'over_code: while (x as usize) < lexed_code.len() {
        for value in &lexed_code[x as usize].top {
            run_top(value, &mut stack, &mut pre_push, &mut negation);
        }
        for value in &lexed_code[x as usize].bottom {
            let jump = run_bottom(value, &mut stack, &mut input, &mut x)?;

            if jump {
                continue 'over_code;
            }
        }

        x += 1;
    }

    //The program will add a % character to the end of the output if I don't do this
    println!();

    Ok(())
}
