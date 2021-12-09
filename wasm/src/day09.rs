use std::collections::HashMap;
use std::iter::Iterator;
use std::vec::Vec;

fn get_neighbors(mx: &Vec<Vec<usize>>, x: usize, y: usize) -> Vec<(usize, usize)> {
  let mut neighbors = vec![];
  if y > 0 {
    neighbors.push((x, y - 1));
  }
  if x > 0 {
    neighbors.push((x - 1, y));
  }
  if x < mx.len() - 1 {
    neighbors.push((x + 1, y));
  }
  if y < mx[0].len() - 1 {
    neighbors.push((x, y + 1));
  }
  neighbors
}

fn get_mins(heights: &Vec<Vec<usize>>) -> Vec<(usize, (usize, usize))> {
  let mut mins = vec![];
  for row_index in 0..heights.len() {
    for col_index in 0..heights[row_index].len() {
      let elem = heights[row_index][col_index];

      let neighbors = get_neighbors(&heights, row_index, col_index);
      if neighbors.iter().all(|(x, y)| elem < heights[*x][*y]) {
        mins.push((elem, (row_index, col_index)));
      }
    }
  }
  mins
}

fn solution1(heights: &Vec<Vec<usize>>) -> usize {
  let mins = get_mins(&heights);

  mins.iter().map(|(e, _)| e + 1).sum()
}

fn print_state(flood_map: &Vec<Vec<usize>>, cntr: usize) {
  println!("Iteration {}", cntr);
  flood_map.iter().for_each(|line| {
    let line_to_print: String = line.iter().map(|&number| number.to_string()).collect();
    println!("{}", line_to_print);
  });
  println!("");
}

fn solution2(heights: &Vec<Vec<usize>>) -> usize {
  let mins = get_mins(&heights);
  let mut flood_map: Vec<Vec<usize>> = heights.iter().map(|row| vec![0; row.len()]).collect();
  mins.iter().enumerate().for_each(|(index, (_, (x, y)))| {
    flood_map[*x][*y] = index + 1;
  });

  // println!("Map {:#?}", flood_map);
  let mut did_update = false;
  // let mut cntr = 0;
  loop {
    did_update = false;
    // cntr += 1;
    // print_state(&flood_map, cntr);
    let mut new_flood_map: Vec<Vec<usize>> = heights.iter().map(|row| vec![0; row.len()]).collect();
    for row_index in 0..heights.len() {
      for col_index in 0..heights[row_index].len() {
        if heights[row_index][col_index] != 9 {
          let mut did_update_local = false;
          let current_flood_map_val = flood_map[row_index][col_index];
          let neighbors = get_neighbors(&flood_map, row_index, col_index);
          neighbors.iter().for_each(|(x, y)| {
            let flood_map_val = flood_map[*x][*y];
            if current_flood_map_val < flood_map_val {
              new_flood_map[row_index][col_index] = flood_map_val;
              did_update_local = true;
            }
          });
          if !did_update_local {
            new_flood_map[row_index][col_index] = current_flood_map_val;
          }
          if did_update_local {
            did_update = true;
          }
        }
      }
    }
    flood_map = new_flood_map;

    if !did_update {
      break;
    }
  }

  // Count the occurrence of each item
  let counter: HashMap<usize, usize> =
    flood_map
      .iter()
      .flatten()
      .fold(HashMap::new(), |mut counter, &element| {
        if element > 0 {
          *counter.entry(element).or_insert(0) += 1;
        }
        //
        counter
      });
  let mut values: Vec<usize> = counter.values().map(|&e| e).collect();
  values.sort_by(|a, b| b.cmp(a));
  let res = values[0..3].iter().fold(1, |prod, x| prod * x);
  res
}

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

  if second {
    solution2(&heights)
  } else {
    solution1(&heights)
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
    assert_eq!(res, 1134);
  }
}
