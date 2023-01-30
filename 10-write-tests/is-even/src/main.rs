pub fn is_even(num: i32) -> bool {
  num % 2 == 0
}

fn main() {

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_true_when_even() {
    assert!(is_even(450) == true);
  }

  #[test]
  fn is_false_when_odd() {
    assert!(is_even(723) == false);
  }
}
