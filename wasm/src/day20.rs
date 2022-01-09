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

fn should_light_up(mapping: &HashSet<usize>, number: usize) -> bool {
  mapping.contains(&number)
}

fn get_bounds(map: &HashSet<(isize, isize)>) -> MinMax {
  map.iter().fold(
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
  )
}

fn display(map: &HashSet<(isize, isize)>) {
  let bounds = get_bounds(map);
  // (bounds.x_min..bounds.x_max).cartesian_product(bounds.y_min..bounds.y_max).for_each
  (bounds.x_min..bounds.x_max + 1).for_each(|x| {
    (bounds.y_min..bounds.y_max + 1)
      .for_each(|y| print!("{}", if map.contains(&(x, y)) { '#' } else { '.' }));
    println!()
  });
}

fn iterate(
  map: &HashSet<(isize, isize)>,
  mapping: &HashSet<usize>,
  bg_lit: bool,
) -> HashSet<(isize, isize)> {
  let bounds = get_bounds(map);

  ((bounds.x_min - 1)..(bounds.x_max + 2))
    .cartesian_product((bounds.y_min - 1)..(bounds.y_max + 2))
    .fold(HashSet::new(), |mut acc, (x_origin, y_origin)| {
      let string_neighbors: String = (-1..2)
        .cartesian_product(-1..2)
        .map(|(x_diff, y_diff)| {
          let x = x_origin + x_diff;
          let y = y_origin + y_diff;
          let is_fringe = (x == bounds.x_min - 1)
            || (x == bounds.x_max + 1)
            || (y == bounds.y_min - 1)
            || (y == bounds.y_max + 1);
          if bg_lit && is_fringe {
            println!(
              "Fringe x_orig {}, y_orig {}, x {}, y {}",
              x_origin, y_origin, x, y
            );
          }
          if (bg_lit && is_fringe) || map.contains(&(x, y)) {
            '1'
          } else {
            '0'
          }
          //
        })
        .collect();
      let int_neighbors = str_to_bin(&string_neighbors);
      if should_light_up(mapping, int_neighbors) {
        acc.insert((x_origin, y_origin));
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

  println!("first");
  display(&map);
  println!();

  for i in 0..2 {
    map = iterate(&map, &mapping, (i % 2 == 1) && mapping.contains(&0));
    println!("Iteration {}", i);
    display(&map);
    println!();
  }
  map.len()
}

#[cfg(test)]
mod tests {
  #[test]
  fn day20_works_first() {
    let input = "#.#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\n#..#.\n#....\n##..#\n..#..\n..###";
    let res = super::main(input, false);
    assert_eq!(res, 35);
  }

  #[test]
  fn day20_works_second() {
    let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    let res = super::main(input, true);
    assert_eq!(res, 5);
  }
}