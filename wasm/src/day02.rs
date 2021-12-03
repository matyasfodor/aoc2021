use std::vec::Vec;

struct State {
  depth: usize,
  horizontal: usize,
  aim: usize,
}

fn direction_2(acc: State, line: &str) -> State {
  let split_line: Vec<&str> = line.split(' ').collect();

  match &split_line[..] {
    ["forward", n] => State {
      horizontal: acc.horizontal + n.parse::<usize>().expect("expected a number"),
      depth: acc.depth + acc.aim * n.parse::<usize>().expect("expected a number"),
      ..acc
    },
    ["down", n] => State {
      aim: acc.aim + n.parse::<usize>().expect("expected a number"),
      ..acc
    },
    ["up", n] => State {
      aim: acc.aim - n.parse::<usize>().expect("expected a number"),
      ..acc
    },
    _ => panic!("Unable to process line {}", line),
  }
}

fn direction_1(acc: State, line: &str) -> State {
  let split_line: Vec<&str> = line.split(' ').collect();

  match &split_line[..] {
    ["forward", n] => State {
      horizontal: acc.horizontal + n.parse::<usize>().expect("expected a number"),
      ..acc
    },
    ["down", n] => State {
      depth: acc.depth + n.parse::<usize>().expect("expected a number"),
      ..acc
    },
    ["up", n] => State {
      depth: acc.depth - n.parse::<usize>().expect("expected a number"),
      ..acc
    },
    _ => panic!("Unable to process line {}", line),
  }
}

pub fn main(s: &str, second: bool) -> usize {
  let end_state = s.split_terminator("\n").fold(
    State {
      depth: 0,
      horizontal: 0,
      aim: 0,
    },
    if second { direction_2 } else { direction_1 },
  );

  end_state.depth * end_state.horizontal
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works_first() {
    let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";
    let res = super::main(input, false);
    assert_eq!(res, 150);
  }

  #[test]
  fn it_works_second() {
    let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";
    let res = super::main(input, true);
    assert_eq!(res, 900);
  }
}
