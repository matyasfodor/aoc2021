use itertools::Itertools;
use std::collections::HashMap;
use std::vec::Vec;

struct State {
  polymer: HashMap<(char, char), u64>,
  instructions: HashMap<(char, char), char>,
}

fn apply_instructions(
  polymer: &HashMap<(char, char), u64>,
  instructions: &HashMap<(char, char), char>,
) -> HashMap<(char, char), u64> {
  polymer.keys().fold(HashMap::new(), |mut acc, key| {
    let elements = if let Some(entity) = instructions.get(key) {
      vec![
        ((key.0, *entity), polymer[key]),
        ((*entity, key.1), polymer[key]),
      ]
    } else {
      vec![(*key, polymer[key])]
    };
    for element in elements {
      *acc.entry(element.0).or_insert(0) += element.1;
    }
    acc
  })
}

fn solution(state: &State, times: u64) -> u64 {
  let mut polymer = state.polymer.clone();
  for i in 0..times {
    polymer = apply_instructions(&polymer, &state.instructions);
    let sum: u64 = polymer.values().sum();
    println!("At step {} the sum is {}", i, sum);
  }
  let char_counts = polymer.iter().fold(HashMap::new(), |mut map, charcter| {
    *map.entry(charcter.0 .0).or_insert(0) += charcter.1;
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
    polymer: format!("{}_", first_line)
      .chars()
      .tuple_windows::<(_, _)>()
      .fold(HashMap::new(), |mut acc, entry| {
        *acc.entry(entry).or_insert(0) += 1;
        acc
      }),
    instructions,
  }
}

pub fn main(s: &str, second: bool) -> u64 {
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
    assert_eq!(res, 2188189693529);
  }
}
