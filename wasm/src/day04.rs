use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;

#[derive(Debug, Default, Clone)]
struct TableCounter {
  row_count: [usize; 5],
  column_count: [usize; 5],
}

fn solution1(guessed_numbers: &Vec<usize>, tables: &Vec<Vec<Vec<usize>>>) -> usize {
  let mut elements_visited = HashSet::<usize>::new();
  let mut table_counters: Vec<TableCounter> = vec![
    TableCounter {
      row_count: [0; 5],
      column_count: [0; 5],
    };
    tables.len()
  ];

  let position_map = tables
    .iter()
    .enumerate()
    .flat_map(|(table_index, table)| {
      table.iter().enumerate().flat_map(move |(row_index, row)| {
        row
          .iter()
          .enumerate()
          .map(move |(column_index, value)| (value, table_index, row_index, column_index))
      })
    })
    .fold(
      HashMap::new(),
      |mut acc, (value, table_index, row_index, column_index)| {
        // acc.insert(value, (table_index, row_index, column_index));
        acc
          .entry(value)
          .or_insert(vec![])
          .push((table_index, row_index, column_index));
        acc
      },
    );

  for number in guessed_numbers {
    elements_visited.insert(*number);
    if let Some(occurrences) = position_map.get(number) {
      for (table_index, row_index, column_index) in occurrences {
        let table = table_counters
          .get_mut(*table_index)
          .expect("Table not found");
        (*table).column_count[*column_index] += 1;
        (*table).row_count[*row_index] += 1;
        if (*table).column_count[*column_index] == 5 || (*table).row_count[*row_index] == 5 {
          let sum_non_marked: usize = tables[*table_index]
            .iter()
            .flat_map(|row| row.iter().filter(|e| !elements_visited.contains(e)))
            .sum();
          return number * sum_non_marked;
        }
      }
    }
  }
  panic!("Couldn't find bingo");
}

fn solution2() -> usize {
  0
}

pub fn main(s: &str, second: bool) -> usize {
  let mut read_iter = s.split_terminator("\n").peekable();
  let guessed_numbers: Vec<usize> = read_iter
    .next()
    .expect("First line not found")
    .split(',')
    .map(|e| e.parse().expect("Expected number"))
    .collect();
  read_iter.next();

  let tables: Vec<Vec<Vec<usize>>> = read_iter
    .fold(vec![vec![]], |mut acc, line| {
      if line.len() == 0 {
        acc.insert(0, vec![]);
      } else {
        // couldn't figure out how to append to the last element, so instead I'm growing the first element
        // Should find a better solution!
        acc[0].push(line);
      }
      acc
    })
    .iter()
    .rev() // reversing, since the lines were grouped in reverse order
    .map(|lines| {
      let ret: Vec<Vec<usize>> = lines
        .iter()
        .map(|line| {
          line
            .split_whitespace()
            .map(|e| e.parse().expect("Not a number "))
            .collect()
        })
        .collect();
      ret
    })
    .collect();

  if second {
    solution2()
  } else {
    solution1(&guessed_numbers, &tables)
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn d04_first() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7\n";
    let res = super::main(input, false);
    assert_eq!(res, 4512);
  }

  #[test]
  fn d04_second() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7\n";
    let res = super::main(input, true);
    assert_eq!(res, 5);
  }
}
