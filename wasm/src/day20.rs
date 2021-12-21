use itertools::Itertools;
use std::cmp;
use std::collections::HashSet;

fn str_to_bin(bin_idx: &str) -> usize {
  usize::from_str_radix(bin_idx, 2).expect("Expected a binary string")
}

struct MinMax {
  x_min: isize,
  x_max: isize,
  y_min: isize,
  y_max: isize,
}

fn should_light_up(mapping: &HashSet<usize>, number: usize, _invert: bool) -> bool {
  mapping.contains(&number)
}

fn iterate(map: &HashSet<(isize, isize)>, mapping: &HashSet<usize>) -> HashSet<(isize, isize)> {
  let bounds = map.iter().fold(
    MinMax {
      x_min: 100,
      x_max: 0,
      y_min: 100,
      y_max: 0,
    },
    |MinMax {
       x_min,
       x_max,
       y_min,
       y_max,
     },
     (x, y)| MinMax {
      x_min: cmp::min(x_min, *x),
      x_max: cmp::max(x_max, *x),
      y_min: cmp::min(y_min, *y),
      y_max: cmp::max(y_max, *y),
    },
  );

  ((bounds.x_min - 1)..(bounds.x_max + 2))
    .cartesian_product((bounds.y_min - 1)..(bounds.y_max + 2))
    .fold(HashSet::new(), |mut acc, (x, y)| {
      let string_neighbors: String = (-1..2)
        .cartesian_product(-1..2)
        .map(|(x_diff, y_diff)| {
          if map.contains(&(x + x_diff, y + y_diff)) {
            '1'
          } else {
            '0'
          }
          //
        })
        .collect();
      let int_neighbors = str_to_bin(&string_neighbors);
      if should_light_up(mapping, int_neighbors, false) {
        acc.insert((x, y));
      }
      acc
    })
  //
}

pub fn main(s: &str, second: bool) -> usize {
  let mut input_iter = s.split_terminator("\n");
  let mapping: HashSet<usize> = input_iter
    .next()
    .expect("Expected to have at last one line")
    .chars()
    .map(|character| match character {
      '.' => false,
      '#' => true,
      _ => panic!("Unknown character"),
    })
    .enumerate()
    .filter(|(_, value)| *value)
    .map(|(idx, _)| idx)
    .collect();
  // let first_line_values = str_to_bin(&first_line_binary_string);
  input_iter.next();
  let mut map: HashSet<(isize, isize)> = input_iter
    .enumerate()
    .flat_map(|(x, line)| {
      line.chars().enumerate().map(move |(y, character)| {
        //
        (x, y, character == '#')
      })
    })
    .fold(HashSet::new(), |mut acc, (x, y, has_light)| {
      if has_light {
        acc.insert((x as isize, y as isize));
      }
      acc
    });
  for _ in 0..2 {
    map = iterate(&map, &mapping);
  }
  map.len()
}

#[cfg(test)]
mod tests {
  #[test]
  fn day20_works_first() {
    let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\n#..#.\n#....\n##..#\n..#..\n..###";
    let res = super::main(input, false);
    assert_eq!(res, 7);
  }

  #[test]
  fn day20_works_second() {
    let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    let res = super::main(input, true);
    assert_eq!(res, 5);
  }
}
