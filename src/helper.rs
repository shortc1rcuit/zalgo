pub fn pop_stack(stack: &mut Vec<u32>) -> Result<u32, &'static str> {
    match stack.pop() {
        Some(x) => Ok(x),
        None => Err("Out of stack values!"),
    }
}

pub fn invert(mut a: u32) -> u32{
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

    inv_a
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
        assert_eq!(invert(a), 13);

        let a = 13;
        assert_eq!(invert(a), 2);

        let a = 2;
        assert_eq!(invert(a), 1);

        let a = 1;
        assert_eq!(invert(a), 0);

        let a = 0;
        assert_eq!(invert(a), 1);
    }
}
