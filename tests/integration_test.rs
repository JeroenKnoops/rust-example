extern crate example;
use example::*;

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(5, 2), 10);
}

#[test]
#[should_panic]
fn test_any_panic() {
    division(1, 0);
}

#[test]
#[should_panic(expected = "Divide result is zero")]
fn test_specific_panic() {
    division(1, 10);
}
