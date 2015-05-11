// #4
// A palindromic number reads the same both ways.
// The largest palindrome made from the product
// of two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.

mod palindrone_checker;
mod product_array_builder;

#[test]
fn products_for_2_digits() {
    assert_eq!(product_array_builder::new(2).len(), 9604);
}

#[test]
fn nine_zero_zero_nine_is_a_palindrone() {
    assert!(palindrone_checker::call(&9009));
}

#[test]
fn odd_numbers_or_not_palidrones() {
    assert!(!palindrone_checker::call(&90093));
}

#[test]
fn nine_zero_one_nine_is_a_palindrone() {
    assert!(!palindrone_checker::call(&9019));
}

#[test]
fn numbers_are_palindrones_sometimes() {
    assert!(palindrone_checker::call(&211112));
    assert!(palindrone_checker::call(&123321));
    assert!(palindrone_checker::call(&44555544));
    assert!(!palindrone_checker::call(&44555549));
}
