use std::process;

pub mod lexer;
use lexer::*;
use cluster::*;

fn main() {
    //Only the diacritics needed to print "Zalgo"
    let unclustered = vec!['̆', '̏', '̐',          //Push o
                           '̆', '̇', '̐',          //Push g
                           '̆', '̌', '̐',          //Push l
                           '̆', '́', '̐',          //Push a
                           '̅', '̊', '̐',          //Push Z
                           '̝', '̝', '̝', '̝', '̝']; //Print 5 chars

    //Converts the characters to numbers to:
    //A) Make characters easier to write in the program
//B) Make 0-F digits easier to use
    let unclustered = unclustered.into_iter()
                                 .map(|x| (x as u32) - 0x300)
                                 .collect();

    let lexed_code = Cluster::new(unclustered);

    if let Err(e) = interpret_code(lexed_code) {
        eprintln!("Runtime error! {}", e);
        process::exit(1);
    }
}

fn interpret_code(lexed_code: Cluster) -> Result<(), &'static str>{
    let mut stack: Vec<u32> = Vec::new();

    //Value used to build a number before it's pushed to the stack
    let mut pre_push = 0;

    let mut result = String::new();

    for value in lexed_code.top {
        if value <= 0xF { //0-F value
            pre_push = (pre_push << 4) + value;
        } else if value == 0x10 { //Push to stack
            stack.push(pre_push);
            pre_push = 0;
        }
    }
    for value in lexed_code.bottom {
        if value == 0x1D { //Pop top of stack and print
            let print_char = match stack.pop() {
                Some(x) => x,
                None => {
                    return Err("Out of stack values!");
                }
            };

            let print_char = match char::from_u32(print_char){
                Some(x) => x,
                None => {
                    return Err("Invalid char value!");
                }
            };

            result = format!("{}{}", result, print_char);
        }
    }

    println!("{}", result);

    Ok(())
}
