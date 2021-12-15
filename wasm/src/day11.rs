use std::collections::BTreeSet;
use std::vec::Vec;

fn copy_map(map: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
  // Dummy progress
  map
    .iter()
    .map(|line| line.iter().map(|e| *e).collect())
    .collect()
}

fn get_nines(map: &Vec<Vec<usize>>) -> BTreeSet<(isize, isize)> {
  let mut ret = BTreeSet::new();
  for (i, row) in map.iter().enumerate() {
    for (j, element) in row.iter().enumerate() {
      // println!("Element {} {}, {}", i, j, element);
      if *element > 9 {
        ret.insert((i as isize, j as isize));
      }
    }
  }
  ret
}

fn progress(map: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
  let mut new_map = copy_map(map);

  for i in 0..new_map.len() {
    for j in 0..new_map[i].len() {
      new_map[i][j] += 1;
    }
  }

  let mut popped = BTreeSet::new();
  let mut nines = get_nines(&new_map);
  // println!("Nines {:#?}", nines);
  while nines.len() > 0 {
    let popped_item = nines.iter().next().cloned().unwrap();
    nines.remove(&popped_item);
    popped.insert(popped_item);
    // println!("  popped_item {:#?}", popped_item);
    for i in -1..2 {
      for j in -1..2 {
        if i == 0 && j == 0 {
          continue;
        }
        let row_index = popped_item.0 + i;
        let column_index = popped_item.1 + j;
        if 0 <= row_index
          && row_index < map.len() as isize
          && 0 <= column_index
          && column_index < map[row_index as usize].len() as isize
        {
          new_map[row_index as usize][column_index as usize] += 1;
          // println!(
          //   "    Progressing ({}, {}) for ({}, {}) value {}",
          //   row_index, column_index, i, j, new_map[row_index as usize][column_index as usize]
          // );
          if new_map[row_index as usize][column_index as usize] > 9
            && !popped.contains(&(row_index, column_index))
          {
            // println!("Adding {:#?} to nines", (row_index, column_index));
            nines.insert((row_index, column_index));
          }
        }
      }
    }
  }

  for i in 0..new_map.len() {
    for j in 0..new_map[i].len() {
      if new_map[i][j] > 9 {
        new_map[i][j] = 0;
      }
    }
  }
  new_map
}

fn count_zeros(map: &Vec<Vec<usize>>) -> usize {
  map.iter().flatten().filter(|e| **e == 0).count()
}

fn solution1(map: &Vec<Vec<usize>>, iters: usize) -> usize {
  let mut local_map = copy_map(map);
  (0..iters).fold(0, |counter, _| {
    local_map = progress(&local_map);
    // println!("Map {:#?}", local_map);
    let new_counter = counter + count_zeros(&local_map);
    new_counter
  })
}

fn solution2(map: &Vec<Vec<usize>>) -> usize {
  let mut cntr = 0;
  let mut local_map = copy_map(map);

  let number_of_octopuses = map.len() * map[0].len();
  loop {
    local_map = progress(&local_map);
    let zeros = count_zeros(&local_map);
    if zeros == number_of_octopuses {
      break;
    }
    cntr += 1
  }
  cntr + 1
}

pub fn main(s: &str, second: bool) -> usize {
  let map: Vec<Vec<usize>> = s
    .split_terminator("\n")
    .map(|line| {
      line
        .chars()
        .map(|e| e.to_digit(10).expect("Expected number") as usize)
        .collect()
    })
    .collect();
  if second {
    solution2(&map)
  } else {
    solution1(&map, 100)
  }
}

#[cfg(test)]
mod tests {
  static TEST_INPUT: &'static str = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526\n";
  #[test]
  fn day11_first() {
    let res = super::main(TEST_INPUT, false);
    assert_eq!(res, 1656);
  }

  #[test]
  fn d11_solution1_10() {
    let map = vec![
      vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
      vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
      vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
      vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
      vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
      vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
      vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
      vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
      vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
      vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ];
    let zeros = super::solution1(&map, 10);
    assert_eq!(zeros, 204);
  }

  #[test]
  fn test_progress_1() {
    let map = vec![
      vec![1, 1, 1, 1, 1],
      vec![1, 9, 9, 9, 1],
      vec![1, 9, 1, 9, 1],
      vec![1, 9, 9, 9, 1],
      vec![1, 1, 1, 1, 1],
    ];
    let new_map = super::progress(&map);
    assert_eq!(
      new_map,
      [
        vec![3, 4, 5, 4, 3],
        vec![4, 0, 0, 0, 4],
        vec![5, 0, 0, 0, 5],
        vec![4, 0, 0, 0, 4],
        vec![3, 4, 5, 4, 3],
      ]
    );
  }

  #[test]
  fn test_progress_2() {
    let map = vec![
      vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
      vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
      vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
      vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
      vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
      vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
      vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
      vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
      vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
      vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ];
    let new_map = super::progress(&map);
    assert_eq!(
      new_map,
      vec![
        vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
        vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
        vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
        vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
        vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
        vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
        vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
        vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
        vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
        vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
      ]
    );
  }

  #[test]
  fn test_progress_3() {
    let map = vec![
      vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
      vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
      vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
      vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
      vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
      vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
      vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
      vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
      vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
      vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
    ];
    let new_map = super::progress(&map);
    assert_eq!(
      new_map,
      vec![
        vec![8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
        vec![5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
        vec![8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
        vec![8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
        vec![8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
        vec![6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
        vec![6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
        vec![0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
        vec![9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
        vec![8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
      ]
    );
  }

  #[test]
  fn day11_second() {
    let res = super::main(TEST_INPUT, true);
    assert_eq!(res, 195);
  }
}
