pub fn main(s: &str, second: bool) -> usize {
  let mut crab_positions: Vec<usize> = s
    .split_terminator(",")
    .map(|e| e.parse::<usize>().expect("expected a number"))
    .collect();
  crab_positions.sort();

  let median: usize = crab_positions[crab_positions.len() / 2];
  let distance = crab_positions
    .iter()
    .map(|position| {
      if median > *position {
        median - position
      } else {
        position - median
      }
    })
    .sum();
  distance
}

#[cfg(test)]
mod tests {
  #[test]
  fn day07_first() {
    let input = "16,1,2,0,4,2,7,1,2,14";
    let res = super::main(input, false);
    assert_eq!(res, 37);
  }

  #[test]
  fn day07_second() {
    let input = "16,1,2,0,4,2,7,1,2,14";
    let res = super::main(input, true);
    assert_eq!(res, 0);
  }
}
