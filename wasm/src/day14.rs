use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::vec::Vec;

#[derive(Debug, Default)]
struct Instruction {
  matcher: (char, char),
  element: char,
}

struct State {
  polymer: HashMap<(char, char), usize>,
  instructions: Vec<Instruction>,
}

fn apply_instructions(
  polymer: &HashMap<(char, char), usize>,
  instructions: &Vec<Instruction>,
) -> HashMap<(char, char), usize> {
  let mut new_polymer = polymer.clone();
  for instruction in instructions {
    // println!("Polymer: {:#?} {:#?}", new_polymer, instruction);

    // if let Some(original_count) = new_polymer.get(&instruction.matcher) {
    let original_count_wrapped = polymer.get(&instruction.matcher);
    if original_count_wrapped.is_none() {
      continue;
    }
    println!("instruction {:#?}", instruction);

    let original_count = *original_count_wrapped.unwrap();
    *new_polymer
      .entry((instruction.matcher.0, instruction.element))
      .or_insert(0) += original_count;
    *new_polymer
      .entry((instruction.element, instruction.matcher.1))
      .or_insert(0) += original_count;
    *new_polymer.entry(instruction.matcher).or_insert(0) = 0
    // }

    // new_polymer[&(instruction.matcher.0, instruction.element)] += original_count;
    // new_polymer[&(instruction.element, instruction.matcher.0)] += original_count;
    // new_polymer[&instruction.matcher] = 0;
  }
  new_polymer
}

fn solution1(state: &State) -> usize {
  let mut polymer = state.polymer.clone();
  for i in 0..1 {
    // println!("Polymer: {:#?}", polymer);
    polymer = apply_instructions(&polymer, &state.instructions);
    let sum: usize = polymer.values().sum();
    println!("At step {} the sum is {}", i, sum);
    println!("Polymer is {:#?}", polymer);
  }
  let char_counts = polymer
    .iter()
    .fold(HashMap::new(), |mut map, ((left, _), value)| {
      *map.entry(left).or_insert(0) += value;
      map
    });
  println!("Char counts value {:#?}", char_counts);
  let max = char_counts.values().max().unwrap();
  let min = char_counts.values().min().unwrap();
  max - min
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
    polymer: format!("{}_", first_line)
      .chars()
      .tuple_windows::<(_, _)>()
      .fold(HashMap::new(), |mut map, element| {
        *map.entry(element).or_insert(0) += 1;
        map
      }),
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
