use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::vec::Vec;

struct State {
  polymer: Vec<char>,
  instructions: HashMap<(char, char), char>,
}

fn apply_instructions(
  polymer: &Vec<char>,
  instructions: &HashMap<(char, char), char>,
) -> Vec<char> {
  let mut ret: Vec<char> = polymer
    .iter()
    .tuple_windows::<(_, _)>()
    .flat_map(|(left, right)| {
      if let Some(new_element) = instructions.get(&(*left, *right)) {
        Vec::from([*left, *new_element])
      } else {
        Vec::from([*left])
      }
    })
    .collect();
  ret.push(*polymer.last().unwrap());
  ret
}

fn solution(state: &State, times: usize) -> usize {
  let mut polymer = state.polymer.clone();
  for i in 0..times {
    // println!("Polymer: {:#?}", polymer);
    polymer = apply_instructions(&polymer, &state.instructions);
    let sum: usize = polymer.len();
    println!("At step {} the sum is {}", i, sum);
    // println!("Polymer is {:#?}", polymer);
  }
  let char_counts = polymer.iter().fold(HashMap::new(), |mut map, charcter| {
    *map.entry(charcter).or_insert(0) += 1;
    map
  });
  println!("Char counts value {:#?}", char_counts);
  let max = char_counts.values().max().unwrap();
  let min = char_counts.values().min().unwrap();
  max - min
}

fn read_input(input: &str) -> State {
  let mut input_iter = input.split_terminator("\n");
  let first_line = input_iter
    .next()
    .expect("Expected to have at last one line");
  input_iter.next();
  let instructions: HashMap<(char, char), char> = input_iter
    .map(
      |line| match &line.split(" -> ").collect::<Vec<&str>>()[..] {
        [left, right] => (
          (left.chars().nth(0).unwrap(), left.chars().nth(1).unwrap()),
          right.chars().nth(0).unwrap(),
        ),
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
    solution(&state, 40)
  } else {
    solution(&state, 10)
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
