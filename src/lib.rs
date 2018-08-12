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
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

