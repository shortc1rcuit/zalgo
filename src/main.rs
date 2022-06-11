use std::process;

fn main() {
    //Only the diacritics needed to print "Zalgo"
    let lexed_code = vec!['̆', '̏', '̐',          //Push o
                          '̆', '̇', '̐',          //Push g
                          '̆', '̌', '̐',          //Push l
                          '̆', '́', '̐',          //Push a
                          '̅', '̊', '̐',          //Push Z
                          '̝', '̝', '̝', '̝', '̝']; //Print 5 chars

    interpret_code(lexed_code);
}

fn interpret_code(code: Vec<char>) {
    let mut stack: Vec<u32> = Vec::new();

    //Value used to build a number before it's pushed to the stack
    let mut pre_push = 0;

    //Converts the characters to numbers to:
    //A) Make characters easier to write in the program
    //B) Make 0-F digits easier to use
    let code: Vec<u32> = code.into_iter()
                   .map(|c| (c as u32) - 0x300)
                   .collect();

    let mut result = String::new();

    for value in code {
        if value <= 0xF { //0-F value
            pre_push = (pre_push << 4) + value;
        } else if value == 0x10 { //Push to stack
            stack.push(pre_push);
            pre_push = 0;
        } else if value == 0x1D { //Pop top of stack and print
            let print_char = match stack.pop() {
                Some(x) => x,
                None => {
                    eprintln!("Out of stack values!");
                    process::exit(1);
                }
            };

            let print_char = match char::from_u32(print_char){
                Some(x) => x,
                None => {
                    eprintln!("Invalid char value!");
                    process::exit(1);
                }
            };

            //print!("{}", print_char);
            result = format!("{}{}", result, print_char);
        }
    }

    println!("{}", result);
}
