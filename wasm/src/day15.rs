use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct SearchState {
  position: (usize, usize),
  cost: usize,
  distance: usize,
  // trail: Vec<(usize, usize)>
}

impl Ord for SearchState {
  fn cmp(&self, other: &Self) -> Ordering {
    (other.cost + other.distance).cmp(&(self.distance + self.cost))
  }
}

impl PartialOrd for SearchState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

// TODO move to a utility
fn get_neighbors(mx: &Vec<Vec<usize>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
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

fn distance(goal: (usize, usize), current: (usize, usize)) -> usize {
  (goal.0 - current.0) + (goal.1 - current.1)
}

fn solution1(grid: &Vec<Vec<usize>>) -> usize {
  let goal = (grid.len() - 1, grid[0].len() - 1);
  let mut heap = BinaryHeap::from([SearchState {
    position: (0, 0),
    distance: distance(goal, (0, 0)),
    cost: 0,
  }]);

  let mut visited_with_cost = HashMap::from([((0, 0), distance(goal, (0, 0)))]);

  while let Some(current_state) = heap.pop() {
    // println!("Heap {:#?}", heap);
    if current_state.position == goal {
      return current_state.cost;
    }
    let neighbors = get_neighbors(&grid, current_state.position);
    for neighbor in neighbors {
      let new_state = SearchState {
        position: neighbor,
        cost: current_state.cost + grid[neighbor.0][neighbor.1],
        distance: distance(goal, neighbor),
      };
      if let Some(found_position) = visited_with_cost.get(&neighbor) {
        if *found_position > new_state.cost + new_state.distance {
          heap.push(new_state);
          visited_with_cost.insert(neighbor, new_state.cost + new_state.distance);
        }
      } else {
        heap.push(new_state);
        visited_with_cost.insert(neighbor, new_state.cost + new_state.distance);
      }
    }
  }
  panic!["Heap should not be empty"];
}

fn solution2(grid: &Vec<Vec<usize>>) -> usize {
  0
}

pub fn main(s: &str, second: bool) -> usize {
  let mut grid: Vec<Vec<usize>> = s
    .split_terminator("\n")
    .map(|line| {
      line
        .chars()
        .map(|e| e.to_digit(10).expect("expected a number") as usize)
        .collect()
    })
    .collect();
  if second {
    solution2(&grid)
  } else {
    solution1(&grid)
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn day15_works() {
    let input = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581\n";
    let res = super::main(input, false);
    assert_eq!(res, 40);
  }

  #[test]
  fn it_works_second() {
    let input = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581\n";
    let res = super::main(input, true);
    assert_eq!(res, 5);
  }
}
