use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::vec::Vec;

struct Instruction {
  matcher: (char, char),
  element: char,
}

struct State {
  polymer: LinkedList<char>,
  instructions: Vec<Instruction>,
}

fn apply_instructions(
  polymer: &LinkedList<char>,
  instructions: &Vec<Instruction>,
) -> LinkedList<char> {
  let mut new_polymer = polymer.clone();
  for instruction in instructions {
    //
    // for (left, right) in polymer.iter().tuple_windows::<(_, _)>() {
    //   if *left == instruction.matcher.0 && *right == instruction.matcher.1 {
    //     //
    //   }
    // }
    let cursor = polymer.cursor_front_mut();
    cursor.move_next();
    while cursor.peek_next().is_some() {
      cursor.move_next();
    }
  }
  new_polymer
}

fn solution1(state: &State) -> usize {
  let mut polymer = state.polymer.clone();
  for _ in 0..10 {
    polymer = apply_instructions(&polymer, &state.instructions);
  }
  0
}

fn solution2(state: &State) -> usize {
  0
}

fn read_input(input: &str) -> State {
  let mut input_iter = input.split_terminator("\n");
  let first_line = input_iter
    .next()
    .expect("Expected to have at last one line");
  input_iter.next();
  let instructions: Vec<Instruction> = input_iter
    .map(
      |line| match &line.split(" -> ").collect::<Vec<&str>>()[..] {
        [left, right] => Instruction {
          matcher: (left.chars().nth(0).unwrap(), left.chars().nth(1).unwrap()),
          element: right.chars().nth(0).unwrap(),
        },
        _ => panic!("This should not happen"),
      },
    )
    .collect();

  State {
    polymer: first_line.chars().collect(),
    instructions,
  }
}

pub fn main(s: &str, second: bool) -> usize {
  let state = read_input(s);
  if second {
    solution2(&state)
  } else {
    solution1(&state)
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn day14_first() {
    let input = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C\n";
    let res = super::main(input, false);
    assert_eq!(res, 1588);
  }

  #[test]
  fn day_14_second() {
    let input = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C\n";
    let res = super::main(input, true);
    assert_eq!(res, 5);
  }
}
