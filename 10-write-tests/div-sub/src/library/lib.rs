/// This function divides two numbers.
///
/// # Example #1: 10 / 2 == 5
///
/// ```
/// let result = div_sub::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Example #2: 6 / 3 = 2
///
/// ```
/// assert_eq!(div_sub::div(6, 3), 2);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// assert_eq!(div_sub::div(13, 0), 13);
/// ```
pub fn div(
  a: i32,
  b: i32,
) -> i32 {
  if b == 0 {
    panic!("Divide-by-zero error");
  }
  a / b
}

/// This function subtracts two numbers.
///
/// # Example #1: 9 - 2 == 7
///
/// ```
/// assert_eq!(div_sub::sub(9, 2), 7);
/// ```
///
/// # Example #2: 6 - 9 == -3
///
/// ```
/// assert_eq!(div_sub::sub(6, 9), -3);
/// ```
pub fn sub(
  a: i32,
  b: i32,
) -> i32 {
  a - b
}
