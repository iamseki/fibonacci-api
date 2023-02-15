pub fn fibonacci_nth(nth: u32) -> u32 {
    // fibonacci number
    let (mut fibo_number, mut count, mut x, mut y) = (0, 1, 0, 1);

    // 0 = 0, 1 = 0, 2 = 1, 3 = 1,
    // 0 1 1 2 3 5 8 13
    // '
    // x = 0
    // y = 1
    // fibo_number = x + y = 1
    // '' x = y  
    // 
    //

    loop {
        if count >= nth {
            break;
        }

        fibo_number = x + y;
        x = y;
        y = fibo_number;
        count += 1;
    }

    fibo_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_nth_0() -> Result<(), String> {
        let nth = 0;
        let expected = 0;
        assert_eq!(fibonacci_nth(nth), expected);
        Ok(())
    }

    #[test]
    fn test_fibonacci_nth_1() -> Result<(), String> {
        let nth = 1;
        let expected = 0;
        assert_eq!(fibonacci_nth(nth), expected);
        Ok(())
    }

    #[test]
    fn test_fibonacci_nth_2() -> Result<(), String> {
        let nth = 2;
        let expected = 1;
        assert_eq!(fibonacci_nth(nth), expected);
        Ok(())
    }

    #[test]
    fn test_fibonacci_nth_3() -> Result<(), String> {
        let nth = 3;
        let expected = 2;
        assert_eq!(fibonacci_nth(nth), expected);
        Ok(())
    }

    #[test]
    fn test_fibonacci_nth_4() -> Result<(), String> {
        let nth = 4;
        let expected = 3;
        assert_eq!(fibonacci_nth(nth), expected);
        Ok(())
    }

    #[test]
    fn test_fibonacci_nth_5() -> Result<(), String> {
        let nth = 5;
        let expected = 5;
        assert_eq!(fibonacci_nth(nth), expected);
        Ok(())
    }

    #[test]
    fn test_fibonacci_nth_6() -> Result<(), String> {
        let nth = 6;
        let expected = 8;
        assert_eq!(fibonacci_nth(nth), expected);
        Ok(())
    }
}
