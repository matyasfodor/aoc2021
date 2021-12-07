pub fn main(s: &str, second: bool) -> usize {
  0
}

#[cfg(test)]
mod tests {
  #[test]
  fn day07_first() {
    let input = "";
    let res = super::main(input, false);
    assert_eq!(res, 0);
  }

  #[test]
  fn day07_second() {
    let input = "";
    let res = super::main(input, true);
    assert_eq!(res, 0);
  }
}
