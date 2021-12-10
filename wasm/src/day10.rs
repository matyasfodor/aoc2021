use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;

fn fin_illegal_charactor(line: &str) -> Option<char> {
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

pub fn main(s: &str, second: bool) -> usize {
  let illetgal_character_map = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
  let sum = s
    .split_terminator("\n")
    .map(fin_illegal_charactor)
    .filter_map(|e| e)
    .map(|illegal_char| illetgal_character_map[&illegal_char])
    .sum();
  sum
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works_first() {
    let input = "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]\n";
    let res = super::main(input, false);
    assert_eq!(res, 26397);
  }

  #[test]
  fn it_works_second() {
    let input = "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]\n";
    let res = super::main(input, true);
    assert_eq!(res, 5);
  }
}
