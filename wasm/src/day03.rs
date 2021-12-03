use std::vec::Vec;

#[derive(Debug, Default)]
struct TreeNode {
  count: usize,
  zeros: Option<Box<TreeNode>>,
  ones: Option<Box<TreeNode>>,
}

impl TreeNode {
  pub fn insert(&mut self, new_val: &str, pointer: usize) {
    self.count += 1;
    if pointer == new_val.len() {
      return;
    }
    let new_pointer = pointer + 1;
    match new_val.chars().nth(pointer).unwrap() {
      '0' => {
        match self.zeros {
          None => {
            let mut new_node = TreeNode {
              count: 0,
              zeros: None,
              ones: None,
            };
            new_node.insert(new_val, new_pointer);
            self.zeros = Some(Box::new(new_node));
          }
          Some(ref mut subnode) => subnode.insert(new_val, new_pointer),
        };
      }
      '1' => {
        match self.ones {
          None => {
            let mut new_node = TreeNode {
              count: 0,
              zeros: None,
              ones: None,
            };
            new_node.insert(new_val, new_pointer);
            self.ones = Some(Box::new(new_node));
          }
          Some(ref mut subnode) => subnode.insert(new_val, new_pointer),
        };
      }
      c => panic!("Character not understood {}", c),
    };
  }

  fn find_trace_private(&self, mut sofar: usize, cb: fn(usize, usize) -> bool) -> usize {
    if self.zeros.is_none() && self.ones.is_none() {
      return sofar;
    }
    let len_zeros = match self.zeros {
      None => 0,
      Some(ref zeros) => zeros.count,
    };
    let len_ones = match self.ones {
      None => 0,
      Some(ref ones) => ones.count,
    };
    sofar = sofar << 1;

    let child = match (len_zeros, len_ones) {
      (0, _) => {
        sofar |= 1;
        self.ones.as_ref()
      }
      (_, 0) => self.zeros.as_ref(),
      _ => {
        if cb(len_ones, len_zeros) {
          sofar |= 1;
          self.ones.as_ref()
        } else {
          self.zeros.as_ref()
        }
      }
    };

    match child {
      Some(ref ptr) => ptr.find_trace_private(sofar, cb),
      None => {
        panic!("we have a problem")
      }
    }
  }

  fn find_trace(&self, cb: fn(usize, usize) -> bool) -> usize {
    self.find_trace_private(0, cb)
  }
}

fn first_solution(s: &str) -> usize {
  let end_state: Option<Vec<f32>> = s.split_terminator("\n").fold(None, |acc, line| {
    let mut new_acc = match acc {
      Some(inner) => inner,
      None => vec![1.0; line.len()],
    };
    for (i, c) in line.chars().enumerate() {
      let multiplier = match c {
        '0' => 1.0 / 2.0,
        '1' => 2.0,
        _ => panic!("Unrecognised character {}", c),
      };
      new_acc[i] *= multiplier;
    }

    Some(new_acc)
  });

  let unwrapped = end_state.unwrap();
  let binary = unwrapped.iter().fold(0, |prev, next| {
    let mut prev_copy = prev;
    prev_copy = prev_copy << 1;

    if *next > 1.0 {
      prev_copy |= 1;
    }
    prev_copy
  });
  let negated_binary = !binary % (1 << (unwrapped.len()));

  binary * negated_binary
}

fn second_solution(s: &str) -> usize {
  let binary_tree = s.split_terminator("\n").fold(
    TreeNode {
      count: 0,
      zeros: None,
      ones: None,
    },
    |mut acc, line| {
      acc.insert(line, 0);
      acc
    },
  );

  let oxigen = binary_tree.find_trace(|a, b| a >= b);

  let co2 = binary_tree.find_trace(|a, b| a < b);

  oxigen * co2
}

pub fn main(s: &str, second: bool) -> usize {
  if second {
    second_solution(s)
  } else {
    first_solution(s)
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works_first() {
    let input =
      "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
    let res = super::main(input, false);
    assert_eq!(res, 198);
  }

  #[test]
  fn tree_works() {
    let input =
      "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
    let res = super::main(input, true);
    assert_eq!(res, 230);
  }
}
