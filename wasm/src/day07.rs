fn solution1(crab_positions: &Vec<usize>) -> usize {
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

// Could be generic
fn double_distance(a: usize, b: usize) -> usize {
  if b > a {
    double_distance(b, a)
  } else {
    let diff = a - b;
    diff.pow(2) + diff
  }
}

fn solution2(crab_positions: &Vec<usize>) -> usize {
  let res = (crab_positions[0]..*crab_positions.last().expect("Expected to have last"))
    .map(|midpoint| {
      crab_positions
        .iter()
        .map(|position| double_distance(*position, midpoint))
        .sum::<usize>()
    })
    .min();

  res.expect("expected answer") / 2
}

pub fn main(s: &str, second: bool) -> usize {
  let mut crab_positions: Vec<usize> = s
    .split_terminator(",")
    .map(|e| e.parse::<usize>().expect("expected a number"))
    .collect();
  crab_positions.sort();

  if second {
    solution2(&crab_positions)
  } else {
    solution1(&crab_positions)
  }
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
  fn test_double_distance() {
    assert_eq!(super::double_distance(12, 5), 56);
    assert_eq!(super::double_distance(5, 12), 56);
  }

  #[test]
  fn day07_second() {
    let input = "16,1,2,0,4,2,7,1,2,14";
    let res = super::main(input, true);
    assert_eq!(res, 168);
  }
}
