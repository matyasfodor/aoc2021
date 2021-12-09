use itertools::Itertools;
use std::iter::Iterator;
use std::vec::Vec;

fn solution1(heights: Vec<Vec<usize>>) -> usize {
  // heights.iter().enumerate().map(|(row_index, row)| {

  // });
  let mut mins = vec![];
  for row_index in 0..heights.len() {
    for col_index in 0..heights[row_index].len() {
      let elem = heights[row_index][col_index];
      // prev row
      if row_index > 0 {
        if let Some(prev_row) = heights.get(row_index - 1) {
          if elem >= prev_row[col_index] {
            continue;
          }
        }
      }

      if col_index > 0 {
        if let Some(west) = heights[row_index].get(col_index - 1) {
          if elem >= *west {
            continue;
          }
        }
      }

      if let Some(east) = heights[row_index].get(col_index + 1) {
        if elem >= *east {
          continue;
        }
      }

      if let Some(next_row) = heights.get(row_index + 1) {
        if elem >= next_row[col_index] {
          continue;
        }
      }
      mins.push(elem);
    }
  }

  mins.iter().map(|e| e + 1).sum()
}

fn solution2(heights: Vec<Vec<usize>>) -> usize {
  0
}

trait Sliding<T> {
  fn sliding_window(&self) -> dyn Iterator<Item = (Option<T>, Option<T>, Option<T>)>;
}

// impl<T> Sliding<T> for Vec<T> {
//   fn sliding_window(&self) -> dyn Iterator<Item=(Option<T>, Option<T>, Option<T>)> {
//     let ret = self.iter().enumerate().map(|(index, element)| {
//       (
//         if index - 1 < 0 {
//           None
//         } else {
//           Some(&self[index - 1])
//         },
//         Some(&self[index]),
//         Some(if index + 1 == self.len() {
//           None
//         } else {
//           Some(&self[index + 1])
//         }),
//       )
//     });
//     Box::new(ret) as Box<dyn Iterator<Item = (Option<T>, Option<T>, Option<T>)>>

//     // for n in 0..self.len() {

//     // }
//     (None, None, None)
//   }
// }

pub fn main(s: &str, second: bool) -> usize {
  let heights: Vec<Vec<usize>> = s
    .split_terminator("\n")
    .map(|line| {
      line
        .chars()
        .map(|e| e.to_digit(10).expect("Expected number") as usize)
        .collect()
    })
    .collect();

  println!("Heights {:#?}", heights);
  if second {
    solution2(heights)
  } else {
    solution1(heights)
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn day09_first() {
    let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678\n";
    let res = super::main(input, false);
    assert_eq!(res, 15);
  }

  #[test]
  fn day09_second() {
    let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678\n";
    let res = super::main(input, true);
    assert_eq!(res, 5);
  }
}
