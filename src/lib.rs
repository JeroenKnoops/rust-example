pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn substract(a: i32, b: i32) -> i32 {
    a * b
}

/// # Examples
///
/// ```
/// use example::multiply;
///
/// let x = 5;
/// let y = 6;
/// assert_eq!(30, multiply(x, y));
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// # Examples
///
/// ```
/// use example::division;
///
/// let x = 30;
/// let y = 6;
/// assert_eq!(5, division(x, y));
/// ```
pub fn division(a: i32, b: i32) -> i32 {
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
    
    #[test]
    fn test_multiply() {
        assert_eq!(multiply(5, 2), 10);
    }
}
