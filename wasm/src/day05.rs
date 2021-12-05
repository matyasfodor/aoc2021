use itertools::Itertools;
use std::cmp;
use std::collections::HashSet;

pub fn main(s: &str, second: bool) -> usize {
  let coordinates = {
    let coordinates_iterator = s.split_terminator("\n").map(|line| {
      line
        .split(" -> ")
        .map(|raw_coord_pair| {
          raw_coord_pair
            .split(",")
            .map(|number| number.parse::<usize>().expect("Expected a number"))
            .collect_tuple::<(_, _)>()
            .expect("Expected coordinates as a tuple of two")
        })
        .collect_tuple::<(_, _)>()
        .expect("Expected coordinate pairs as a tuple of two")
    });
    if !second {
      coordinates_iterator
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .collect::<Vec<((usize, usize), (usize, usize))>>()
    } else {
      coordinates_iterator.collect::<Vec<((usize, usize), (usize, usize))>>()
    }
  };
  // Filter out the coordinates that are either horizontal ar vertical

  let (_, visited_nodes_twice) = coordinates.iter().fold(
    (
      HashSet::<(usize, usize)>::new(),
      HashSet::<(usize, usize)>::new(),
    ),
    |(mut visited_nodes_once, mut visited_nodes_twice), ((x1, y1), (x2, y2))| {
      let loop_iterator: Vec<(usize, usize)> = if *x1 == *x2 || *y1 == *y2 {
        let x_low = cmp::min(*x1, *x2);
        let x_max = cmp::max(*x1, *x2);
        let y_low = cmp::min(*y1, *y2);
        let y_max = cmp::max(*y1, *y2);
        print!("horizontal / vertical \t\t");
        (x_low..x_max + 1)
          .cartesian_product(y_low..y_max + 1)
          .collect()
      } else {
        println!("Diagonal \t\t");
        let x_range = if *x1 < *x2 {
          Box::new(*x1..*x2 + 1) as Box<dyn Iterator<Item = usize>>
        } else {
          Box::new((*x2..*x1 + 1).rev()) as Box<dyn Iterator<Item = usize>>
        };
        let y_range = if *y1 < *y2 {
          Box::new(*y1..*y2 + 1) as Box<dyn Iterator<Item = usize>>
        } else {
          Box::new((*y2..*y1 + 1).rev()) as Box<dyn Iterator<Item = usize>>
        };
        x_range.zip(y_range).collect()
      };
      for (x, y) in loop_iterator.iter() {
        // print!("({},{}),", x, y);
        if visited_nodes_once.contains(&(*x, *y)) {
          visited_nodes_twice.insert((*x, *y));
        } else {
          visited_nodes_once.insert((*x, *y));
        }
      }
      // println!("");
      (visited_nodes_once, visited_nodes_twice)
    },
  );

  visited_nodes_twice.len()
}

#[cfg(test)]
mod tests {
  #[test]
  fn d04_first() {
    let input = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2\n1,1 -> 3,3\n9,7 -> 7,9\n";
    let res = super::main(input, false);
    assert_eq!(res, 5);
  }

  #[test]
  fn d04_second() {
    let input = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2\n1,1 -> 3,3\n9,7 -> 7,9\n";
    let res = super::main(input, true);
    assert_eq!(res, 15);
  }
}
