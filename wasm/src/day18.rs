use itertools::Itertools;
use std::fmt;
use std::vec::Vec;

#[derive(Debug, Clone)]
enum GraphElement {
  Simple(usize),
  Embed(Graph),
}

impl GraphElement {
  fn value(&self) -> usize {
    match self {
      GraphElement::Simple(value) => *value,
      GraphElement::Embed(graph) => graph.value(),
    }
  }
}

#[derive(Debug, Default, Clone)]
struct Graph {
  children: Vec<GraphElement>,
}

impl Graph {
  fn value(&self) -> usize {
    3 * self.children[0].value() + 2 * self.children[1].value()
  }

  fn from_inner(values: &[usize], levels: &[usize], level: usize) -> Self {
    // println!("values {:?} levels {:?} level {}", values, levels, level);
    let mut iter = levels.iter();
    let mut sum_elements = 0;
    let mut cntr = 0;
    while sum_elements != level {
      // println!(
      //   "Sum elements {} cntr {} level {}",
      //   sum_elements, cntr, level
      // );
      sum_elements += iter.next().unwrap();
      cntr += 1;
    }
    let first_element = if cntr == 1 {
      GraphElement::Simple(values[0])
    } else {
      GraphElement::Embed(Graph::from_inner(
        &values[..cntr],
        &levels[..cntr],
        level >> 1,
      ))
    };
    let second_element = if levels.len() - cntr == 1 {
      GraphElement::Simple(values[values.len() - 1])
    } else {
      GraphElement::Embed(Graph::from_inner(
        &values[cntr..],
        &levels[cntr..],
        level >> 1,
      ))
    };
    Self {
      children: vec![first_element, second_element],
    }
  }
}

impl fmt::Display for Graph {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    assert!(self.children.len() == 2);
    let string = self
      .children
      .iter()
      .map(|child| match child {
        GraphElement::Simple(number) => format!("{}", number),
        GraphElement::Embed(graph_node) => format!("{}", graph_node),
      })
      .join(",");
    write!(f, "[{}]", string)
  }
}

impl From<&Number> for Graph {
  fn from(number: &Number) -> Self {
    let binary_levels = number
      .level
      .iter()
      .map(|level| 0b10000000 >> level)
      .collect::<Vec<_>>();
    // println!("Levels {:?}", number);
    Self::from_inner(&number.values[..], &binary_levels[..], 0b1000000)
  }
}

#[derive(Debug, Default, Clone)]
struct Number {
  values: Vec<usize>,
  level: Vec<usize>,
}

impl Number {
  fn add(&self, other: &Self) -> Self {
    let pre_reduce = &Self {
      values: self
        .values
        .iter()
        .chain(other.values.iter())
        .map(|e| *e)
        .collect(),
      level: self
        .level
        .iter()
        .chain(other.level.iter())
        .map(|e| *e + 1)
        .collect(),
    };
    // println!("pre-reduce {}", pre_reduce.raw_display());
    Number::reduce(pre_reduce)
  }

  fn reduce_inner(number: &Self) -> (bool, Self) {
    // println!("Levels {:#?}", number.level);
    if let Some(explode_index) = number.level.iter().position(|x| *x == 5) {
      // println!(" #### Explode index {}", explode_index);
      // explode
      let new_number: Number = number
        .values
        .iter()
        .zip_eq(number.level.iter())
        .enumerate()
        .fold(
          Self {
            values: Vec::new(),
            level: Vec::new(),
          },
          |mut acc, (index, (value, level))| {
            if explode_index.checked_sub(1) == Some(index) {
              let new_value = value + number.values[explode_index];
              // println!(
              //   " ### First element active value {} level {} new value {}",
              //   value, level, new_value
              // );
              acc.values.push(new_value);
              acc.level.push(*level);
            } else if index == explode_index {
              acc.values.push(0);
              acc.level.push(level - 1);
            } else if index == explode_index + 1 {
              // println!("Skip exploded element {}", index)
              // Don't do anything
            } else if index == explode_index + 2 {
              let new_value = value + number.values[explode_index + 1];
              // println!(
              //   " ### First element active value {} level {} new value {}",
              //   value, level, new_value
              // );
              acc.values.push(new_value);
              acc.level.push(*level);
            } else {
              acc.values.push(*value);
              acc.level.push(*level);
            }
            acc
          },
        );
      (true, new_number)
    } else if let Some(split_index) = number.values.iter().position(|x| *x >= 10) {
      // split
      let new_number: Self = number
        .values
        .iter()
        .zip_eq(number.level.iter())
        .enumerate()
        .fold(
          Self {
            values: Vec::new(),
            level: Vec::new(),
          },
          |mut acc, (index, (value, level))| {
            if index == split_index {
              let first_val = value / 2;
              let second_val = value / 2 + if (value % 2) == 1 { 1 } else { 0 };
              acc.values.push(first_val);
              acc.values.push(second_val);
              acc.level.push(level + 1);
              acc.level.push(level + 1);
            } else {
              acc.values.push(*value);
              acc.level.push(*level);
            }
            acc
          },
        );
      (true, new_number)
    } else {
      (false, number.clone())
    }
  }

  fn reduce(inp_number: &Self) -> Self {
    let mut number = inp_number.clone();
    loop {
      let resp = Self::reduce_inner(&number);
      number = resp.1;
      if !resp.0 {
        break;
      }
    }
    number.clone()
  }

  fn magnitude(&self) -> usize {
    self
      .values
      .iter()
      .next()
      .expect("To have at least one element")
      * 3
      + self
        .values
        .iter()
        .rev()
        .next()
        .expect("To have at least one element")
        * 2
  }

  fn raw_display(&self) -> String {
    format!(
      "Number {{\n  values: {:?},\n  level: {:?}\n}}",
      self.values, self.level
    )
  }

  // Static parser method
  fn parse_shellfish_number(line: &str) -> Self {
    line
      .chars()
      .fold(
        (
          Self {
            values: Vec::new(),
            level: Vec::new(),
          },
          0,
        ),
        |(mut acc, level), new_char| match new_char {
          '0'..='9' => {
            acc
              .values
              .push(new_char.to_digit(10).expect("Should be a number") as usize);
            acc.level.push(level);
            (acc, level)
          }
          '[' => (acc, level + 1),
          ']' => (acc, level - 1),
          ',' => (acc, level),
          _ => panic!("Unexpected character {}", new_char),
        },
      )
      .0
  }
}

impl fmt::Display for Number {
  // Doesn't work properly, fixing it would probably be too expensive.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut level = self.level.clone();
    level.push(0);
    level.insert(0, 0);
    let string: String = self
      .values
      .iter()
      .zip_eq(level.iter().tuple_windows::<(_, _, _)>())
      .fold(
        "".to_string(),
        |acc, (value, (prev_level, current_level, next_level))| {
          let prefix = if prev_level < current_level {
            std::iter::repeat("[")
              .take(current_level - prev_level)
              .collect()
          } else {
            "".to_string()
          };
          let postfix = if current_level < next_level {
            std::iter::repeat("]")
              .take(next_level - current_level)
              .collect()
          } else {
            "".to_string()
          };

          format!("{}{}{}{},", acc, prefix, value, postfix).to_string()
        },
      );
    write!(f, "{}", string)
  }
}

pub fn main(s: &str, second: bool) -> usize {
  let numbers_to_sum: Vec<Number> = s
    .split_terminator("\n")
    .map(|line| Number::parse_shellfish_number(line))
    .collect();

  if second {
    numbers_to_sum
      .iter()
      .cartesian_product(numbers_to_sum.iter())
      .map(|(a, b)| {
        if a.values == b.values && a.level == b.level {
          0
        } else {
          Graph::from(&a.add(b)).value()
        }
      })
      .max()
      .expect("Expected to have max item")
  } else {
    let final_number = numbers_to_sum
      .iter()
      .fold(None, |acc: Option<Number>, element| match acc {
        Some(number) => Some(number.add(element)),
        None => Some(element.clone()),
      });
    // println!("{:?}", final_number);
    let graph = Graph::from(&final_number.expect("Should be a Number"));
    graph.value()
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn d18_magnitude() {
    let number = super::Number::parse_shellfish_number("[9,1]");
    assert_eq!(number.magnitude(), 29);
  }

  #[test]
  fn d18_format() {
    let raw_string = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]";
    let number = super::Number::parse_shellfish_number(&raw_string);
    // println!("{}", number.raw_display());
    assert_eq!(format!("{}", number), raw_string);
  }

  #[test]
  fn d18_simple() {
    let number1 = super::Number::parse_shellfish_number("[1,9]");
    let number2 = super::Number::parse_shellfish_number("[8,7]");
    let resp = number1.add(&number2);
    assert_eq!(resp.raw_display(), "");
  }

  #[test]
  fn d18_graph() {
    let graph = super::Graph {
      children: vec![
        super::GraphElement::Simple(0),
        super::GraphElement::Embed(super::Graph {
          children: vec![
            super::GraphElement::Simple(9),
            super::GraphElement::Simple(5),
          ],
        }),
      ],
    };
    assert_eq!(format!("{}", graph), "[0,[9,5]]")
  }

  #[test]
  fn d18_graph_parse_simple() {
    let number = super::Number::parse_shellfish_number("[1,9]");
    let graph = super::Graph::from(&number);
    assert_eq!(format!("{}", graph), "[1,9]");
  }

  #[test]
  fn d18_graph_parse_more_complex() {
    let number = super::Number::parse_shellfish_number("[0,[9,5]]");
    let graph = super::Graph::from(&number);
    assert_eq!(format!("{}", graph), "[0,[9,5]]");
  }

  #[test]
  fn d18_graph_parse_very_complex() {
    let string = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
    let number = super::Number::parse_shellfish_number(string);
    let graph = super::Graph::from(&number);
    assert_eq!(format!("{}", graph), string);
  }

  #[test]
  fn d18_reduce_first() {
    let string = "[[[[[9,8],1],2],3],4]";
    let number = super::Number::parse_shellfish_number(string);
    let reduced_number = super::Number::reduce(&number);
    let graph = super::Graph::from(&reduced_number);
    assert_eq!(format!("{}", graph), "[[[[0,9],2],3],4]");
  }

  #[test]
  fn d18_full() {
    let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
    let resp = super::main(input, false);
    assert_eq!(resp, 4140);
  }

  #[test]
  fn d18_full_second() {
    let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
    let resp = super::main(input, true);
    assert_eq!(resp, 3993);
  }

  #[test]
  fn d18_solution() {
    let input = "[4,[3,[9,[9,0]]]]\n[[[7,6],[2,[2,5]]],[5,[[7,3],8]]]\n[4,[4,6]]\n[[0,[5,6]],[[[1,3],[2,7]],[[0,6],4]]]\n[6,[[3,[6,0]],3]]\n[[7,[9,[8,5]]],[6,7]]\n[[[[2,6],1],2],[3,[8,4]]]\n[4,[[[5,4],[2,7]],[[8,0],[2,3]]]]\n[[[[4,3],2],[[3,6],[2,5]]],[[[3,7],8],0]]\n[[[8,[0,7]],1],[[9,[3,9]],9]]\n[[[[3,0],[1,3]],[[0,9],8]],[[[7,2],9],[[1,4],[3,5]]]]\n[[[[9,6],[4,4]],[1,3]],[[4,3],[[6,4],[8,4]]]]\n[[[1,2],[[7,6],[2,3]]],[[4,6],[4,2]]]\n[[[4,8],[[5,8],1]],[2,3]]\n[[[5,2],[3,[5,7]]],[[2,9],5]]\n[[[6,[3,2]],[2,6]],[[8,[4,2]],[[5,2],7]]]\n[[[[2,6],[0,1]],[7,[3,6]]],[[1,6],[[7,9],0]]]\n[[[0,3],[8,1]],[[[9,0],3],[0,2]]]\n[[8,[[7,1],[4,7]]],[[0,[1,3]],[8,2]]]\n[[[[2,3],4],[[0,8],[9,0]]],[1,[[5,3],4]]]\n[[[[7,2],2],[[1,3],[8,3]]],[4,[[7,9],[0,6]]]]\n[[[[2,2],[3,4]],[[1,5],[4,3]]],[6,[[7,2],1]]]\n[1,[[[5,7],0],[9,[8,8]]]]\n[[[[9,2],[0,9]],[4,[7,8]]],[[4,8],[[1,8],[4,9]]]]\n[[[[4,7],2],2],4]\n[1,[[2,[4,2]],1]]\n[[[[7,2],[3,8]],[0,[1,3]]],[[[4,4],[2,4]],[8,2]]]\n[[[[1,0],[0,5]],2],[[9,[5,0]],[[1,6],5]]]\n[4,[[[8,1],[1,4]],[7,[1,3]]]]\n[[[6,[0,4]],[[4,6],[2,4]]],[9,[1,5]]]\n[[[[3,6],[3,3]],1],[0,[[8,8],2]]]\n[[7,[5,[2,6]]],[[[7,9],6],[0,[3,6]]]]\n[[[[6,7],4],[[2,9],2]],3]\n[[[7,[1,7]],[5,4]],[[[1,1],[0,1]],5]]\n[[6,[[1,0],6]],[0,[6,[0,5]]]]\n[[[[2,4],[4,6]],9],[4,[[8,0],7]]]\n[[[[9,9],[5,7]],[9,[8,6]]],[[3,[2,3]],0]]\n[[0,[1,[5,3]]],[3,[8,[3,4]]]]\n[[[[4,3],8],[2,9]],[[1,[6,5]],[[5,7],2]]]\n[[[0,[7,4]],[9,[9,6]]],[[8,[5,5]],[[6,4],1]]]\n[[[[7,3],[7,9]],[8,[6,2]]],[[8,[4,5]],[[6,4],[6,7]]]]\n[[7,[[9,0],[9,0]]],[[[0,8],2],[8,[8,3]]]]\n[4,[7,[5,6]]]\n[7,[[[3,8],8],3]]\n[[[4,[6,6]],0],[9,0]]\n[[[[7,4],8],8],[[0,1],[[0,0],[2,4]]]]\n[7,[1,[[9,4],[3,6]]]]\n[[[[2,8],9],[[8,6],[2,2]]],[[[5,1],9],[2,[0,7]]]]\n[8,7]\n[[[[0,8],4],[[9,9],[9,9]]],[[[4,3],[1,0]],[6,8]]]\n[[[[8,3],[8,9]],1],[[4,[1,0]],[[4,0],[2,3]]]]\n[[[[4,7],[1,3]],[6,9]],[[1,0],[[1,8],5]]]\n[[2,[4,[6,5]]],[3,[[9,9],5]]]\n[[[[7,6],4],9],[8,[4,5]]]\n[[[0,[6,6]],[7,[8,9]]],[[[0,0],[3,4]],[4,[1,8]]]]\n[[[9,[7,0]],[5,8]],[6,[[5,0],[0,6]]]]\n[[[[4,0],[1,9]],[7,[3,6]]],[[2,[8,6]],[[2,8],[8,2]]]]\n[[[9,6],8],[[[5,5],[4,8]],0]]\n[[[[1,7],1],2],[[[6,8],3],[[3,3],5]]]\n[3,[5,[[3,8],6]]]\n[3,[[[9,6],[5,8]],[9,2]]]\n[[6,1],[6,4]]\n[[2,6],[[[1,2],2],8]]\n[[[[1,7],[3,6]],[2,[0,2]]],[[3,0],9]]\n[1,[[0,[4,9]],5]]\n[[[[5,5],[5,2]],[0,[6,4]]],8]\n[0,[7,[[6,9],[6,0]]]]\n[[[[2,2],[4,7]],[[7,4],6]],[[0,[1,7]],[[3,2],6]]]\n[[9,8],0]\n[[[[5,4],[4,8]],2],[3,[8,9]]]\n[[[[7,0],8],5],[2,6]]\n[[[5,[0,8]],5],[[[5,0],[1,8]],[[0,2],7]]]\n[[[[9,4],8],[[6,5],4]],[[5,[8,9]],[4,[0,4]]]]\n[[[[3,6],7],[[9,3],7]],[7,[[8,3],9]]]\n[[[[0,7],5],[[5,7],2]],[[2,[9,5]],[[7,7],[5,0]]]]\n[[[[7,5],2],[8,6]],[[2,[6,2]],[5,[3,1]]]]\n[[9,[9,1]],6]\n[[[0,7],[[5,9],2]],3]\n[[[9,3],[8,8]],[0,[4,5]]]\n[[[[6,2],5],[4,[3,1]]],[9,[2,8]]]\n[[[1,[9,4]],[[0,0],2]],[[1,[2,1]],[[7,8],[3,2]]]]\n[[[[0,6],[8,9]],[[4,7],[5,6]]],[[[1,4],[8,7]],[4,6]]]\n[[[[6,4],[1,5]],[0,8]],[[[9,7],[1,2]],[9,4]]]\n[[[[4,5],[0,7]],[9,[1,8]]],[[[5,0],6],7]]\n[[[0,[6,9]],[5,[5,6]]],7]\n[[4,5],[[7,[6,5]],1]]\n[[[7,9],[6,7]],[4,1]]\n[[[[9,6],1],[[3,1],[9,7]]],[1,[7,1]]]\n[[[0,[2,0]],5],[[8,[7,6]],[[7,3],4]]]\n[[[6,[1,7]],[9,[2,7]]],3]\n[[[6,[8,2]],5],[4,[[1,3],[5,1]]]]\n[[[4,[3,3]],[4,[2,4]]],[5,4]]\n[[[1,6],[4,[4,0]]],[[8,[2,2]],[[8,1],[4,7]]]]\n[[2,0],[[2,1],[[4,8],[2,7]]]]\n[9,[[8,4],0]]\n[[1,6],[[5,[1,3]],[9,[0,9]]]]\n[[[0,[3,5]],3],[[2,[8,0]],[[2,0],[4,3]]]]\n[[[1,[1,9]],[9,[7,9]]],[[2,2],[[6,7],[0,7]]]]\n[[[4,6],[[6,2],[0,9]]],[[1,0],[1,[6,7]]]]\n[9,[[[0,1],4],[[9,3],3]]]\n";
    let resp = super::main(input, false);
    assert_eq!(resp, 4140);
  }

  #[test]
  fn d18_solution_second() {
    let input = "[4,[3,[9,[9,0]]]]\n[[[7,6],[2,[2,5]]],[5,[[7,3],8]]]\n[4,[4,6]]\n[[0,[5,6]],[[[1,3],[2,7]],[[0,6],4]]]\n[6,[[3,[6,0]],3]]\n[[7,[9,[8,5]]],[6,7]]\n[[[[2,6],1],2],[3,[8,4]]]\n[4,[[[5,4],[2,7]],[[8,0],[2,3]]]]\n[[[[4,3],2],[[3,6],[2,5]]],[[[3,7],8],0]]\n[[[8,[0,7]],1],[[9,[3,9]],9]]\n[[[[3,0],[1,3]],[[0,9],8]],[[[7,2],9],[[1,4],[3,5]]]]\n[[[[9,6],[4,4]],[1,3]],[[4,3],[[6,4],[8,4]]]]\n[[[1,2],[[7,6],[2,3]]],[[4,6],[4,2]]]\n[[[4,8],[[5,8],1]],[2,3]]\n[[[5,2],[3,[5,7]]],[[2,9],5]]\n[[[6,[3,2]],[2,6]],[[8,[4,2]],[[5,2],7]]]\n[[[[2,6],[0,1]],[7,[3,6]]],[[1,6],[[7,9],0]]]\n[[[0,3],[8,1]],[[[9,0],3],[0,2]]]\n[[8,[[7,1],[4,7]]],[[0,[1,3]],[8,2]]]\n[[[[2,3],4],[[0,8],[9,0]]],[1,[[5,3],4]]]\n[[[[7,2],2],[[1,3],[8,3]]],[4,[[7,9],[0,6]]]]\n[[[[2,2],[3,4]],[[1,5],[4,3]]],[6,[[7,2],1]]]\n[1,[[[5,7],0],[9,[8,8]]]]\n[[[[9,2],[0,9]],[4,[7,8]]],[[4,8],[[1,8],[4,9]]]]\n[[[[4,7],2],2],4]\n[1,[[2,[4,2]],1]]\n[[[[7,2],[3,8]],[0,[1,3]]],[[[4,4],[2,4]],[8,2]]]\n[[[[1,0],[0,5]],2],[[9,[5,0]],[[1,6],5]]]\n[4,[[[8,1],[1,4]],[7,[1,3]]]]\n[[[6,[0,4]],[[4,6],[2,4]]],[9,[1,5]]]\n[[[[3,6],[3,3]],1],[0,[[8,8],2]]]\n[[7,[5,[2,6]]],[[[7,9],6],[0,[3,6]]]]\n[[[[6,7],4],[[2,9],2]],3]\n[[[7,[1,7]],[5,4]],[[[1,1],[0,1]],5]]\n[[6,[[1,0],6]],[0,[6,[0,5]]]]\n[[[[2,4],[4,6]],9],[4,[[8,0],7]]]\n[[[[9,9],[5,7]],[9,[8,6]]],[[3,[2,3]],0]]\n[[0,[1,[5,3]]],[3,[8,[3,4]]]]\n[[[[4,3],8],[2,9]],[[1,[6,5]],[[5,7],2]]]\n[[[0,[7,4]],[9,[9,6]]],[[8,[5,5]],[[6,4],1]]]\n[[[[7,3],[7,9]],[8,[6,2]]],[[8,[4,5]],[[6,4],[6,7]]]]\n[[7,[[9,0],[9,0]]],[[[0,8],2],[8,[8,3]]]]\n[4,[7,[5,6]]]\n[7,[[[3,8],8],3]]\n[[[4,[6,6]],0],[9,0]]\n[[[[7,4],8],8],[[0,1],[[0,0],[2,4]]]]\n[7,[1,[[9,4],[3,6]]]]\n[[[[2,8],9],[[8,6],[2,2]]],[[[5,1],9],[2,[0,7]]]]\n[8,7]\n[[[[0,8],4],[[9,9],[9,9]]],[[[4,3],[1,0]],[6,8]]]\n[[[[8,3],[8,9]],1],[[4,[1,0]],[[4,0],[2,3]]]]\n[[[[4,7],[1,3]],[6,9]],[[1,0],[[1,8],5]]]\n[[2,[4,[6,5]]],[3,[[9,9],5]]]\n[[[[7,6],4],9],[8,[4,5]]]\n[[[0,[6,6]],[7,[8,9]]],[[[0,0],[3,4]],[4,[1,8]]]]\n[[[9,[7,0]],[5,8]],[6,[[5,0],[0,6]]]]\n[[[[4,0],[1,9]],[7,[3,6]]],[[2,[8,6]],[[2,8],[8,2]]]]\n[[[9,6],8],[[[5,5],[4,8]],0]]\n[[[[1,7],1],2],[[[6,8],3],[[3,3],5]]]\n[3,[5,[[3,8],6]]]\n[3,[[[9,6],[5,8]],[9,2]]]\n[[6,1],[6,4]]\n[[2,6],[[[1,2],2],8]]\n[[[[1,7],[3,6]],[2,[0,2]]],[[3,0],9]]\n[1,[[0,[4,9]],5]]\n[[[[5,5],[5,2]],[0,[6,4]]],8]\n[0,[7,[[6,9],[6,0]]]]\n[[[[2,2],[4,7]],[[7,4],6]],[[0,[1,7]],[[3,2],6]]]\n[[9,8],0]\n[[[[5,4],[4,8]],2],[3,[8,9]]]\n[[[[7,0],8],5],[2,6]]\n[[[5,[0,8]],5],[[[5,0],[1,8]],[[0,2],7]]]\n[[[[9,4],8],[[6,5],4]],[[5,[8,9]],[4,[0,4]]]]\n[[[[3,6],7],[[9,3],7]],[7,[[8,3],9]]]\n[[[[0,7],5],[[5,7],2]],[[2,[9,5]],[[7,7],[5,0]]]]\n[[[[7,5],2],[8,6]],[[2,[6,2]],[5,[3,1]]]]\n[[9,[9,1]],6]\n[[[0,7],[[5,9],2]],3]\n[[[9,3],[8,8]],[0,[4,5]]]\n[[[[6,2],5],[4,[3,1]]],[9,[2,8]]]\n[[[1,[9,4]],[[0,0],2]],[[1,[2,1]],[[7,8],[3,2]]]]\n[[[[0,6],[8,9]],[[4,7],[5,6]]],[[[1,4],[8,7]],[4,6]]]\n[[[[6,4],[1,5]],[0,8]],[[[9,7],[1,2]],[9,4]]]\n[[[[4,5],[0,7]],[9,[1,8]]],[[[5,0],6],7]]\n[[[0,[6,9]],[5,[5,6]]],7]\n[[4,5],[[7,[6,5]],1]]\n[[[7,9],[6,7]],[4,1]]\n[[[[9,6],1],[[3,1],[9,7]]],[1,[7,1]]]\n[[[0,[2,0]],5],[[8,[7,6]],[[7,3],4]]]\n[[[6,[1,7]],[9,[2,7]]],3]\n[[[6,[8,2]],5],[4,[[1,3],[5,1]]]]\n[[[4,[3,3]],[4,[2,4]]],[5,4]]\n[[[1,6],[4,[4,0]]],[[8,[2,2]],[[8,1],[4,7]]]]\n[[2,0],[[2,1],[[4,8],[2,7]]]]\n[9,[[8,4],0]]\n[[1,6],[[5,[1,3]],[9,[0,9]]]]\n[[[0,[3,5]],3],[[2,[8,0]],[[2,0],[4,3]]]]\n[[[1,[1,9]],[9,[7,9]]],[[2,2],[[6,7],[0,7]]]]\n[[[4,6],[[6,2],[0,9]]],[[1,0],[1,[6,7]]]]\n[9,[[[0,1],4],[[9,3],3]]]\n";
    let resp = super::main(input, true);
    assert_eq!(resp, 4706);
  }
}
