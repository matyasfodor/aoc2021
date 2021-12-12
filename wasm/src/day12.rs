use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;

static START: &'static str = "start";
static END: &'static str = "end";

fn pathfinder(
  graph: &HashMap<&str, std::vec::Vec<&str>>,
  node: &str,
  visited: &HashSet<&str>,
  can_visit_twice: &HashSet<&str>,
  trail: &str,
) -> HashSet<String> {
  if node == END {
    return HashSet::from([trail.to_string()]);
  }
  graph
    .get(node)
    .unwrap_or(&vec![])
    .iter()
    .fold(HashSet::new(), |mut set_of_trails, neighbor| {
      if !visited.contains(neighbor) || can_visit_twice.contains(neighbor) {
        let mut child_visited = visited.clone();
        if neighbor.chars().next().unwrap().is_lowercase() {
          child_visited.insert(neighbor);
        }

        let child_can_visit_twice = if visited.contains(neighbor) {
          can_visit_twice
            .difference(&HashSet::from([*neighbor]))
            .map(|e| *e)
            .collect()
        } else {
          can_visit_twice.clone()
        };

        set_of_trails.extend(pathfinder(
          graph,
          neighbor,
          &child_visited,
          &child_can_visit_twice,
          format!("{},{}", trail, neighbor).as_ref(),
        ));
      }
      set_of_trails
    })
}

fn solution1(graph: &HashMap<&str, std::vec::Vec<&str>>) -> usize {
  pathfinder(
    graph,
    START,
    &HashSet::from([START]),
    &HashSet::new(),
    format!("{}", START).as_ref(),
  )
  .len()
}

fn solution2(graph: &HashMap<&str, std::vec::Vec<&str>>) -> usize {
  let small_caves = graph
    .keys()
    .filter(|cave| {
      cave
        .chars()
        .next()
        .expect("Expected cave to have at least one char")
        .is_lowercase()
    })
    .filter(|cave| *cave != &START && *cave != &END);

  // println!("Small caves {:#?}", small_caves);

  let trails = small_caves.fold(HashSet::new(), |mut trails, small_cave| {
    trails.extend(pathfinder(
      graph,
      START,
      &HashSet::from([START]),
      &HashSet::from([*small_cave]),
      format!("{}", START).as_ref(),
    ));
    trails
  });
  // println!("Trails {:#?}", trails);

  trails.len()
}

pub fn main(s: &str, second: bool) -> usize {
  let graph = s
    .split_terminator("\n")
    .map(|line| match &line.split("-").collect::<Vec<&str>>()[..] {
      [first, second] => [(*first, *second), (*second, *first)],
      _ => panic!("Line did not match"),
    })
    .flatten()
    .into_group_map();
  if second {
    solution2(&graph)
  } else {
    solution1(&graph)
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn day12_first() {
    let input = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end\n";
    let res = super::main(input, false);
    assert_eq!(res, 10);
  }

  #[test]
  fn day12_first_bigger() {
    let input = "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc\n";
    let res = super::main(input, false);
    assert_eq!(res, 19);
  }

  #[test]
  fn day12_first_biggest() {
    let input = "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW\n";
    let res = super::main(input, false);
    assert_eq!(res, 226);
  }

  #[test]
  fn day_12_second() {
    let input = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end\n";
    let res = super::main(input, true);
    assert_eq!(res, 36);
  }
}
