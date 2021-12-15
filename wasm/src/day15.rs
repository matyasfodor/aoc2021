use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct SearchState {
  position: (usize, usize),
  cost: usize,
  distance: usize,
}

impl SearchState {
  fn total_cost(&self) -> usize {
    self.cost + self.distance
  }
}

impl Ord for SearchState {
  fn cmp(&self, other: &Self) -> Ordering {
    other.total_cost().cmp(&(self.total_cost()))
  }
}

impl PartialOrd for SearchState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

struct Grid {
  original_grid: Vec<Vec<usize>>,
  repeater: usize,
  original_bounds: (usize, usize),
  extended_bounds: (usize, usize),
  goal: (usize, usize),
}

impl Grid {
  fn new(mx: &Vec<Vec<usize>>, repeater: usize) -> Self {
    Grid {
      // not sure if there's a better way to get hold of the reference here
      original_grid: mx.clone(),
      repeater,
      original_bounds: (mx.len(), mx[0].len()),
      extended_bounds: (mx.len() * repeater, mx[0].len() * repeater),
      goal: (mx.len() * repeater - 1, mx[0].len() * repeater - 1),
    }
  }

  fn get_neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if y > 0 {
      neighbors.push((x, y - 1));
    }
    if x > 0 {
      neighbors.push((x - 1, y));
    }
    if x < (self.extended_bounds.0) - 1 {
      neighbors.push((x + 1, y));
    }
    if y < (self.extended_bounds.1) - 1 {
      neighbors.push((x, y + 1));
    }
    neighbors
  }

  fn get_element(&self, (x, y): (usize, usize)) -> usize {
    // Cache would be useful here..
    let offset_x = x / self.original_bounds.0;
    let offset_y = y / self.original_bounds.1;
    if offset_x >= self.repeater || offset_y >= self.repeater {
      panic!(
        "Index {} {} out of bounds for {} {} with repeater {}",
        x, y, self.original_bounds.0, self.original_bounds.1, self.repeater
      )
    }
    let original_value = self.original_grid[x % self.original_bounds.0][y % self.original_bounds.1];
    let increment = offset_x + offset_y;
    let unbounded_value = original_value + increment;
    // Values are wrapped back to 1 instead of 0, threfore values are shifted 1 down,
    // and modulo is taken with 9 (instead of 10), finally the values are shifted back up.
    let bounded_value = (unbounded_value - 1) % 9 + 1;
    bounded_value
  }

  fn distance(&self, (x, y): (usize, usize)) -> usize {
    (self.goal.0 - x) + (self.goal.1 - y)
  }
}

fn solution(grid: &Grid) -> usize {
  let mut heap = BinaryHeap::from([SearchState {
    position: (0, 0),
    distance: grid.distance((0, 0)),
    cost: 0,
  }]);

  let mut visited_with_cost = HashMap::from([((0, 0), grid.distance((0, 0)))]);

  while let Some(current_state) = heap.pop() {
    // println!("Heap {:#?}", heap);
    if grid.distance(current_state.position) == 0 {
      return current_state.cost;
    }
    let neighbors = grid.get_neighbors(current_state.position);
    for neighbor in neighbors {
      let new_state = SearchState {
        position: neighbor,
        cost: current_state.cost + grid.get_element(neighbor),
        distance: grid.distance(neighbor),
      };
      if let Some(found_position) = visited_with_cost.get(&neighbor) {
        if *found_position > new_state.total_cost() {
          heap.push(new_state);
          visited_with_cost.insert(neighbor, new_state.total_cost());
        }
      } else {
        heap.push(new_state);
        visited_with_cost.insert(neighbor, new_state.total_cost());
      }
    }
  }
  panic!["Heap should not be empty"];
}

pub fn main(s: &str, second: bool) -> usize {
  let grid: Vec<Vec<usize>> = s
    .split_terminator("\n")
    .map(|line| {
      line
        .chars()
        .map(|e| e.to_digit(10).expect("expected a number") as usize)
        .collect()
    })
    .collect();
  solution(&Grid::new(&grid, if second { 5 } else { 1 }))
}

#[cfg(test)]
mod tests {
  #[test]
  fn day15_first() {
    let input = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581\n";
    let res = super::main(input, false);
    assert_eq!(res, 40);
  }

  #[test]
  fn test_grid() {
    let raw_grid = vec![vec![8]];
    let grid = super::Grid::new(&raw_grid, 5);
    let values: Vec<Vec<usize>> = (0..5)
      .map(|x| (0..5).map(|y| grid.get_element((x, y))).collect())
      .collect();
    assert_eq!(
      values,
      vec![
        vec![8, 9, 1, 2, 3],
        vec![9, 1, 2, 3, 4],
        vec![1, 2, 3, 4, 5],
        vec![2, 3, 4, 5, 6],
        vec![3, 4, 5, 6, 7],
      ]
    )
  }

  #[test]
  fn day15_second() {
    let input = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581\n";
    let res = super::main(input, true);
    assert_eq!(res, 315);
  }
}
