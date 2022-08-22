pub fn pop_stack(stack: &mut Vec<i32>) -> Result<i32, &'static str> {
    match stack.pop() {
        Some(x) => Ok(x),
        None => Err("Out of stack values!"),
    }
}

pub fn invert(mut a: i32) -> Result<i32, &'static str> {
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

pub fn cycle_up(stack: &mut Vec<i32>, size: i32, offset: i32) -> Result<(), &'static str> {
    if size < 0 {
        return Err("Given amount of elements to shift is negative");
    } else if size > stack.len() as i32 {
        return Err("Given amount of elements to shift is larger than the stack");
    }

    if offset < 0 {
        return Err("Given amount to shift elements by is negative");
    }

    let length = stack.len();

    stack[(length - size as usize)..].rotate_right(offset as usize);

    Ok(())
}

pub fn cycle_down(stack: &mut Vec<i32>, size: i32, offset: i32) -> Result<(), &'static str> {
    if size < 0 {
        return Err("Given amount of elements to shift is negative");
    } else if size > stack.len() as i32 {
        return Err("Given amount of elements to shift is larger than the stack");
    }

    if offset < 0 {
        return Err("Given amount to shift elements by is negative");
    }

    let length = stack.len();

    stack[(length - size as usize)..].rotate_left(offset as usize);

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

        _ = cycle_up(&mut stack, 3, 1);
        assert_eq!(stack, vec![1, 2, 5, 3, 4]);

        _ = cycle_down(&mut stack, 4, 1);
        assert_eq!(stack, vec![1, 5, 3, 4, 2]);

        assert_eq!(
            cycle_up(&mut stack, 6, 1),
            Err("Given amount of elements to shift is larger than the stack")
        );
        assert_eq!(
            cycle_down(&mut stack, 6, 1),
            Err("Given amount of elements to shift is larger than the stack")
        );

        assert_eq!(
            cycle_up(&mut stack, -3, 1),
            Err("Given amount of elements to shift is negative")
        );
        assert_eq!(
            cycle_down(&mut stack, -3, 1),
            Err("Given amount of elements to shift is negative")
        );

        assert_eq!(
            cycle_up(&mut stack, 3, -1),
            Err("Given amount to shift elements by is negative")
        );
        assert_eq!(
            cycle_down(&mut stack, 3, -1),
            Err("Given amount to shift elements by is negative")
        );
    }
}
