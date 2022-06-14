use std::process;

pub mod lexer;
use cluster::*;
use lexer::*;

fn main() {
    let base_code = "A\u{0310}\u{030A} \u{0305}\u{0310}\u{0301}\u{0306}\u{0310}\u{030C}\u{031D}\u{0306}\u{031D}\u{031D}B\u{0310}\u{0307}\u{0306}\u{0310}\u{030F}\u{0306}\u{031D}\u{031D}";
    let lexed_code = match lex_text(base_code) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Compile Error! {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = interpret_code(lexed_code) {
        eprintln!("Runtime error! {}", e);
        process::exit(1);
    }
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
