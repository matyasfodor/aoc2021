use itertools::Itertools;
use std::vec::Vec;

pub fn main(s: &str, second: bool) -> usize {
  let mut numbers_to_sum: Vec<usize> = s
    .split_terminator("\n")
    .map(|e| e.parse::<usize>().expect("expected a number"))
    .collect();
  if second {
    numbers_to_sum = numbers_to_sum
      .iter()
      .tuple_windows::<(_, _, _)>()
      .map(|tuple| [*tuple.0, *tuple.1, *tuple.2].iter().sum())
      .collect();
  }
  let res = numbers_to_sum
    .iter()
    .tuple_windows::<(_, _)>()
    .map(|(a, b)| if a < b { 1 } else { 0 })
    .sum();
  return res;
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works_first() {
    let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    let res = super::main(input, false);
    assert_eq!(res, 7);
  }

  #[test]
  fn it_works_second() {
    let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    let res = super::main(input, true);
    assert_eq!(res, 5);
  }
}
