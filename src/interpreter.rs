use crate::lexer::cluster::token::*;
use std::io::{self, Write};

pub fn run_top(value: &TopSet, stack: &mut Vec<i32>, pre_push: &mut i32, negation: &mut i32) {
    match *value {
        //0 - F value
        TopSet::Number(a) => {
            *pre_push = (*pre_push << 4) + a;
        }
        //Used to push negative values
        TopSet::Negate => {
            if *pre_push == 0 {
                *negation *= -1;
            }
        }
        //Push to stack
        TopSet::Push => {
            stack.push(*pre_push * *negation);
            *pre_push = 0;
            *negation = 1;
        }
        //Pop top off stack
        TopSet::Pop => {
            stack.pop();
        }
    }
}

pub fn run_bottom(
    value: &BottomSet,
    stack: &mut Vec<i32>,
    input: &mut String,
    x: &mut i32,
) -> Result<bool, &'static str> {
    match *value {
        //Pop the top of the stack
        //If 0, skip the next cluster
        BottomSet::If => {
            let if_check = pop_stack(stack)?;

            if if_check == 0 {
                *x += 2;
            } else {
                *x += 1;
            }

            return Ok(true);
        }
        BottomSet::Print => {
            //Pop top of stack and print
            let print_char = pop_stack(stack)?;

            let print_char = match char::from_u32(print_char as u32) {
                Some(x) => x,
                None => {
                    return Err("Invalid char value!");
                }
            };

            print!("{}", print_char);
        }
        BottomSet::Input => {
            if input.is_empty() {
                io::stdout().flush().expect("Failed to flush buffer");

                io::stdin().read_line(input).expect("Failed to read line");

                //The input will have a newline at the end, so it's removed
                //A null byte is added to the end so that it is possible to tell when the
                //string ends
                input.pop();

                if input.ends_with('\r') {
                    input.pop();
                }

                *input = format!("{}{}", input, "\0")
            }

            let mut input_chars = input.chars();
            //This is ok as the previous if statement
            //makes sure the string has something in it
            stack.push(input_chars.next().unwrap() as i32);
            *input = input_chars.collect();
        }
        BottomSet::Dup => {
            //Duplicate top value of stack
            let single = pop_stack(stack)?;

            stack.push(single);
            stack.push(single);
        }
        BottomSet::Jump => {
            let position = pop_stack(stack)?;

            *x += position;

            if *x < 0 {
                return Err("Jumped to position before the program");
            }

            return Ok(true);
        }
        BottomSet::Add => {
            let a = pop_stack(stack)?;
            let b = pop_stack(stack)?;

            stack.push(a + b);
        }
        BottomSet::Sub => {
            let a = pop_stack(stack)?;
            let b = pop_stack(stack)?;

            stack.push(b - a);
        }
        BottomSet::Mul => {
            let a = pop_stack(stack)?;
            let b = pop_stack(stack)?;

            stack.push(a * b);
        }
        BottomSet::Div => {
            let a = pop_stack(stack)?;
            let b = pop_stack(stack)?;

            stack.push(b / a);
        }
        BottomSet::Mod => {
            let a = pop_stack(stack)?;
            let b = pop_stack(stack)?;

            stack.push(b % a);
        }
        BottomSet::And => {
            let a = pop_stack(stack)?;
            let b = pop_stack(stack)?;

            stack.push(b & a);
        }
        BottomSet::Or => {
            let a = pop_stack(stack)?;
            let b = pop_stack(stack)?;

            stack.push(b | a);
        }
        BottomSet::Not => {
            let a = pop_stack(stack)?;

            stack.push(invert(a)?);
        }
        BottomSet::Bsl => {
            let a = pop_stack(stack)?;
            let b = pop_stack(stack)?;

            stack.push(b << a);
        }
        BottomSet::Bsr => {
            let a = pop_stack(stack)?;
            let b = pop_stack(stack)?;

            stack.push(b >> a);
        }
        BottomSet::Equal => {
            let a = pop_stack(stack)?;
            let b = pop_stack(stack)?;

            stack.push({
                if a == b {
                    1
                } else {
                    0
                }
            });
        }
        BottomSet::Greater => {
            let a = pop_stack(stack)?;
            let b = pop_stack(stack)?;

            stack.push({
                if b > a {
                    1
                } else {
                    0
                }
            });
        }
        BottomSet::Less => {
            let a = pop_stack(stack)?;
            let b = pop_stack(stack)?;

            stack.push({
                if b < a {
                    1
                } else {
                    0
                }
            });
        }
        BottomSet::Cycle => {
            let a = pop_stack(stack)?;
            let b = pop_stack(stack)?;

            cycle(stack, b, a)?;
        }
    }

    Ok(false)
}

fn pop_stack(stack: &mut Vec<i32>) -> Result<i32, &'static str> {
    match stack.pop() {
        Some(x) => Ok(x),
        None => Err("Out of stack values!"),
    }
}

fn invert(mut a: i32) -> Result<i32, &'static str> {
    if a < 0 {
        return Err("Inverting a negative value");
    }

    //I only want to invert the significant bits
    //e.g: inverting 18 should only invert the 5 end bits
    let mut i = 1;
    let mut inv_a = 0;

    loop {
        if a % 2 == 0 {
            inv_a += i;
        }

        i <<= 1;
        a >>= 1;

        if a == 0 {
            break;
        }
    }

    Ok(inv_a)
}

fn cycle(stack: &mut Vec<i32>, size: i32, offset: i32) -> Result<(), &'static str> {
    if size < 0 {
        return Err("Given amount of elements to shift is negative");
    } else if size > stack.len() as i32 {
        return Err("Given amount of elements to shift is larger than the stack");
    }

    let length = stack.len();

    if offset >= 0 {
        stack[(length - size as usize)..].rotate_right(offset as usize);
    } else {
        stack[(length - size as usize)..].rotate_left((-offset) as usize);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_stack_test() {
        let mut stack = vec![1, 2, 3];
        assert_eq!(pop_stack(&mut stack), Ok(3));
        assert_eq!(pop_stack(&mut stack), Ok(2));
        assert_eq!(pop_stack(&mut stack), Ok(1));
        assert_eq!(pop_stack(&mut stack), Err("Out of stack values!"));
    }

    #[test]
    fn invert_test() {
        let a = 18;
        assert_eq!(invert(a), Ok(13));

        let a = 13;
        assert_eq!(invert(a), Ok(2));

        let a = 2;
        assert_eq!(invert(a), Ok(1));

        let a = 1;
        assert_eq!(invert(a), Ok(0));

        let a = 0;
        assert_eq!(invert(a), Ok(1));

        let a = -1;
        assert_eq!(invert(a), Err("Inverting a negative value"));
    }

    #[test]
    fn cycle_test() {
        let mut stack = vec![1, 2, 3, 4, 5];

        _ = cycle(&mut stack, 3, 1);
        assert_eq!(stack, vec![1, 2, 5, 3, 4]);

        assert_eq!(
            cycle(&mut stack, 6, 1),
            Err("Given amount of elements to shift is larger than the stack")
        );

        assert_eq!(
            cycle(&mut stack, -3, 1),
            Err("Given amount of elements to shift is negative")
        );

        assert_eq!(
            cycle(&mut stack, 3, -1),
            Err("Given amount to shift elements by is negative")
        );
    }
}
