use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;

fn solution01(input: &Vec<Vec<Vec<&str>>>) -> usize {
  input
    .iter()
    .map(|expression| expression[1].clone())
    .fold(0, |sum, line| {
      sum
        + line
          .iter()
          .filter(|line_entry| match line_entry.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
          })
          .count()
    })
}

fn sort_item(input: &str) -> String {
  let mut chars: Vec<char> = input.chars().collect();
  chars.sort_by(|a, b| b.cmp(a));
  String::from_iter(chars.iter().rev())
}

fn solve_line(digits: &Vec<&str>, output_digits: &Vec<&str>) -> usize {
  let sorted_digist = digits.iter().map(|digit| sort_item(digit));
  // let sorted_outputs = output_digits.iter().map(|digit| sort_item(digit));
  let grouped_digits = sorted_digist
    .map(|digit_chars| (digit_chars.len(), digit_chars))
    .into_group_map();

  let one: HashSet<char> = match &grouped_digits[&2][..] {
    [element] => element.chars().collect(),
    _ => panic!("There should be one and only one 1"),
  };

  let seven: HashSet<char> = match &grouped_digits[&3][..] {
    [element] => element.chars().collect(),
    _ => panic!("There should be one and only one 7"),
  };

  let four: HashSet<char> = match &grouped_digits[&4][..] {
    [element] => element.chars().collect(),
    _ => panic!("There should be one and only one 4"),
  };

  let eight: HashSet<char> = match &grouped_digits[&7][..] {
    [element] => element.chars().collect(),
    _ => panic!("There should be one and only one 8"),
  };

  // 6 digits: 0, 6, 9

  // 4 is in 9 but not in 6 or 0
  // 7 is in 0 but not in 6

  let digits_of_length_six: Vec<HashSet<char>> = grouped_digits[&6]
    .iter()
    .map(|digit| digit.chars().collect())
    .collect();

  let (nine_digits, digits_of_length_six): (Vec<HashSet<char>>, Vec<HashSet<char>>) =
    digits_of_length_six
      .into_iter()
      .partition(|digit| four.is_subset(&digit));
  let nine: HashSet<char> = match &nine_digits[..] {
    [element] => element.clone(),
    _ => panic!(
      "There should be one and only one 9 {:#?} {:#?}",
      nine_digits, digits_of_length_six
    ),
  };

  let (zero_digits, digits_of_length_six): (Vec<HashSet<char>>, Vec<HashSet<char>>) =
    digits_of_length_six
      .into_iter()
      .partition(|digit| seven.is_subset(&digit));
  let zero: HashSet<char> = match &zero_digits[..] {
    [element] => element.clone(),
    _ => panic!("There should be one and only one 0"),
  };

  let six: HashSet<char> = match &digits_of_length_six[..] {
    [element] => element.clone(),
    _ => panic!("There should be one and only one 6"),
  };
  // 5: digits: 2, 3, 5

  // 7 is in 3 but not in 2 or 5
  // 5 is in 9 but not in 2

  let digits_of_length_five: Vec<HashSet<char>> = grouped_digits[&5]
    .iter()
    .map(|digit| digit.chars().collect())
    .collect();

  let (three_digits, digits_of_length_five): (Vec<HashSet<char>>, Vec<HashSet<char>>) =
    digits_of_length_five
      .into_iter()
      .partition(|digit| seven.is_subset(&digit));
  let three: HashSet<char> = match &three_digits[..] {
    [element] => element.clone(),
    _ => panic!("There should be one and only one 3",),
  };

  let (five_digits, digits_of_length_five): (Vec<HashSet<char>>, Vec<HashSet<char>>) =
    digits_of_length_five
      .into_iter()
      .partition(|digit| digit.is_subset(&nine));
  let five: HashSet<char> = match &five_digits[..] {
    [element] => element.clone(),
    _ => panic!("There should be one and only one 5"),
  };

  let two: HashSet<char> = match &digits_of_length_five[..] {
    [element] => element.clone(),
    _ => panic!("There should be one and only one 6"),
  };

  // println!(
  //   "Numbers are 1: {:#?} 7: {:#?} 4: {:#?} 8: {:#?} 6: {:#?} 9: {:#?} 0: {:#?} 3: {:#?} 5: {:#?} 2: {:#?}",
  //   one, seven, four, eight, six, nine, zero, three, five, two
  // );
  let mapping: HashMap<String, usize> = vec![
    (sort_item(zero.into_iter().collect::<String>().as_ref()), 0),
    (sort_item(one.into_iter().collect::<String>().as_ref()), 1),
    (sort_item(two.into_iter().collect::<String>().as_ref()), 2),
    (sort_item(three.into_iter().collect::<String>().as_ref()), 3),
    (sort_item(four.into_iter().collect::<String>().as_ref()), 4),
    (sort_item(five.into_iter().collect::<String>().as_ref()), 5),
    (sort_item(six.into_iter().collect::<String>().as_ref()), 6),
    (sort_item(seven.into_iter().collect::<String>().as_ref()), 7),
    (sort_item(eight.into_iter().collect::<String>().as_ref()), 8),
    (sort_item(nine.into_iter().collect::<String>().as_ref()), 9),
  ]
  .into_iter()
  .collect();

  // println!("Mapping is {:#?}", mapping);
  output_digits.iter().fold(0, |sum, digit| {
    // (**digit).sort();
    let sorted_digit = sort_item(digit);
    let new_digit = mapping
      .get(&sorted_digit)
      .unwrap_or_else(|| panic!("Expect sorted_digit {} {:#?}", sorted_digit, mapping));
    10 * sum + new_digit
  })
}

fn solution02(input: &Vec<Vec<Vec<&str>>>) -> usize {
  input
    .iter()
    .map(|line| solve_line(&line[0], &line[1]))
    .sum()
}

pub fn main(s: &str, second: bool) -> usize {
  let numbers_to_sum: Vec<Vec<Vec<&str>>> = s
    .split_terminator("\n")
    .map(|e| {
      e.split("|")
        .map(|line| line.trim().split_terminator(" ").collect())
        .collect()
    })
    .collect();

  if second {
    solution02(&numbers_to_sum)
  } else {
    solution01(&numbers_to_sum)
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn day_08_solution1() {
    let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\n";
    let res = super::main(input, false);
    assert_eq!(res, 26);
  }

  #[test]
  fn sort_item() {
    assert_eq!(super::sort_item("befsd"), "bdefs");
  }

  #[test]
  fn day_08_solution2() {
    let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\n";
    let res = super::main(input, true);
    assert_eq!(res, 61229);
  }
}
