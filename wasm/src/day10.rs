use std::collections::HashMap;
use std::collections::VecDeque;

fn find_illegal_charactor(line: &str) -> Option<char> {
  let closing_pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

  let mut queue: VecDeque<char> = VecDeque::new();
  for line_char in line.chars() {
    if closing_pairs.contains_key(&line_char) {
      queue.push_back(line_char);
    } else {
      if let Some(last_element) = queue.pop_back() {
        if closing_pairs[&last_element] != line_char {
          return Some(line_char);
        }
      } else {
        panic!("Not sure what to do");
      }
    }
  }
  None
}

fn find_closing_sequence(line: &str) -> Option<String> {
  let closing_pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

  let mut queue: VecDeque<char> = VecDeque::new();
  for line_char in line.chars() {
    if closing_pairs.contains_key(&line_char) {
      queue.push_back(line_char);
    } else {
      if let Some(last_element) = queue.pop_back() {
        if closing_pairs[&last_element] != line_char {
          return None;
        }
      } else {
        panic!("Not sure what to do");
      }
    }
  }
  Some(
    queue
      .iter()
      .rev()
      .map(|element| closing_pairs[element])
      .collect(),
  )
}

fn completion_cost(completion: &str) -> u64 {
  let cost_map = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

  completion
    .chars()
    .fold(0, |sum, new_char| sum * 5 + cost_map[&new_char])
}

pub fn main(s: &str, second: bool) -> u64 {
  let lines = s.split_terminator("\n");

  if !second {
    let illetgal_character_map = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    lines
      .map(find_illegal_charactor)
      .filter_map(|e| e)
      .map(|illegal_char| illetgal_character_map[&illegal_char])
      .sum()
  } else {
    let mut line_costs: Vec<u64> = lines
      .filter_map(find_closing_sequence)
      .map(|e| completion_cost(&e))
      .collect();
    println!("Line costs {:#?}", line_costs);
    line_costs.sort();
    line_costs[line_costs.len() / 2]
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn day10_first() {
    let input = "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]\n";
    let res = super::main(input, false);
    assert_eq!(res, 26397);
  }

  #[test]
  fn test_find_closing_sequence() {
    let res = super::find_closing_sequence("[({(<(())[]>[[{[]{<()<>>");
    let unwrapped = res.expect("Should be some");
    assert_eq!(unwrapped, "}}]])})]".to_owned());
  }

  #[test]
  fn test_completion_cost() {
    assert_eq!(super::completion_cost("}}]])})]"), 288957);
  }

  #[test]
  fn day_10_second() {
    let input = "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]\n";
    let res = super::main(input, true);
    assert_eq!(res, 288957);
  }
}
