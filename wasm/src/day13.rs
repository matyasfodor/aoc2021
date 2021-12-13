use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
  Horizontal,
  Vertical,
}

#[derive(Debug, Default)]
struct State {
  coordinates: HashSet<(usize, usize)>,
  instructions: Vec<(Direction, usize)>,
}

fn fold_instruction(
  coordinates: &HashSet<(usize, usize)>,
  direction: Direction,
  number: usize,
) -> HashSet<(usize, usize)> {
  coordinates
    .iter()
    .map(|(x, y)| {
      if direction == Direction::Horizontal && y > &number {
        (*x, 2 * number - y)
      } else if direction == Direction::Vertical && x > &number {
        (2 * number - x, *y)
      } else {
        (*x, *y)
      }
      //
    })
    .collect()
}

fn solution1(state: &State) -> usize {
  let first_instruction = state
    .instructions
    .iter()
    .next()
    .expect("Expected to have at least one instruction");
  let new_coordinates =
    fold_instruction(&state.coordinates, first_instruction.0, first_instruction.1);

  new_coordinates.len()
}

fn solution2(state: &State) -> usize {
  0
}

fn read_input(s: &str) -> State {
  let mut state = State {
    coordinates: HashSet::new(),
    instructions: vec![],
  };
  let mut coordinates_over = false;
  let instruction_matcher =
    Regex::new(r"^fold along (?P<direction>[x|y])=(?P<number>\d+)$").unwrap();
  for line in s.split_terminator("\n") {
    if line == "" {
      coordinates_over = true;
      continue;
    }
    if !coordinates_over {
      match &line.split(",").collect::<Vec<&str>>()[..] {
        [x, y] => {
          // println!("line {}", line);
          state.coordinates.insert((
            x.parse::<usize>().expect("Should be a number"),
            y.parse::<usize>().expect("Should be a number"),
          ));
        }
        _ => panic!("Line does not match the shape"),
      }
    } else {
      let captures = instruction_matcher
        .captures(line)
        .expect("Should contain instructions");
      state.instructions.push((
        if &captures["direction"] == "x" {
          Direction::Vertical
          // Direction::Horizontal
        } else {
          Direction::Horizontal

          // Direction::Vertical
        },
        captures["number"]
          .parse::<usize>()
          .expect("This should have been a number"),
      ));
    }
  }
  state
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
  fn day13_first() {
    let input = "6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\n\nfold along y=7\nfold along x=5\n";
    let res = super::main(input, false);
    assert_eq!(res, 17);
  }

  #[test]
  fn day_13_second() {
    let input = "6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\n\nfold along y=7\nfold along x=5\n";
    let res = super::main(input, true);
    assert_eq!(res, 36);
  }
}
