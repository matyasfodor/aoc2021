use itertools::Itertools;
use regex::Regex;
use std::vec::Vec;

fn is_within_inner(
  x: isize,
  y: isize,
  x_speed: isize,
  y_speed: isize,
  x1: isize,
  x2: isize,
  y1: isize,
  y2: isize,
) -> bool {
  if x1 <= x && x <= x2 && y1 <= y && y <= y2 {
    true
  } else {
    if x2 < x || y < y1 {
      false
    } else {
      let new_x_speed = if x_speed < 0 {
        x_speed + 1
      } else if x_speed > 0 {
        x_speed - 1
      } else {
        0
      };
      is_within_inner(
        x + x_speed,
        y + y_speed,
        new_x_speed,
        y_speed - 1,
        x1,
        x2,
        y1,
        y2,
      )
    }
  }
}

fn is_within(x_speed: isize, y_speed: isize, x1: isize, x2: isize, y1: isize, y2: isize) -> bool {
  is_within_inner(0, 0, x_speed, y_speed, x1, x2, y1, y2)
}

fn max_y(original_y: isize) -> isize {
  (original_y * (original_y + 1)) / 2
}

fn solution(x1: isize, x2: isize, y1: isize, y2: isize, second: bool) -> isize {
  let passing_coords: Vec<(isize, isize)> = (0..200)
    .cartesian_product(-200..200)
    .filter(|(x, y)| is_within(*x, *y, x1, x2, y1, y2))
    .collect();

  println!("Vector {:#?}", passing_coords);

  if second {
    passing_coords.len() as isize
  } else {
    passing_coords
      .iter()
      .map(|(_, y)| max_y(*y))
      .max()
      .expect("expected to have a max")
  }
}

pub fn main(s: &str, second: bool) -> isize {
  let matcher = Regex::new(
    r"^target area: x=(?P<x1>-?\d+)\.\.(?P<x2>-?\d+), y=(?P<y1>-?\d+)\.\.(?P<y2>-?\d+)$",
  )
  .unwrap();
  let result = matcher.captures(s).expect("Regex should match");
  let x1 = result["x1"].parse::<isize>().expect("Should be a number");
  let x2 = result["x2"].parse::<isize>().expect("Should be a number");
  let y1 = result["y1"].parse::<isize>().expect("Should be a number");
  let y2 = result["y2"].parse::<isize>().expect("Should be a number");
  println!("x1 {} x2 {} y1 {} y2 {}", x1, x2, y1, y2);
  solution(x1, x2, y1, y2, second)
}

#[cfg(test)]
mod tests {
  #[test]
  fn day17_first_test() {
    let input = "target area: x=20..30, y=-10..-5";
    let res = super::main(input, false);
    assert_eq!(res, 45);
  }

  #[test]
  fn day17_first_actual() {
    let input = "target area: x=60..94, y=-171..-136";
    let res = super::main(input, false);
    assert_eq!(res, 14535);
  }

  #[test]
  fn day17_is_within() {
    let res = super::is_within(6, 9, 20, 30, -10, -5);
    assert_eq!(res, true);
  }

  #[test]
  fn day17_is_within_1() {
    let res = super::is_within(7, -1, 20, 30, -10, -5);
    assert_eq!(res, true);
  }

  #[test]
  fn day17_is_within_2() {
    let res = super::is_within(6, 0, 20, 30, -10, -5);
    assert_eq!(res, true);
  }

  #[test]
  fn day17_second_test() {
    let input = "target area: x=20..30, y=-10..-5";
    let res = super::main(input, true);
    assert_eq!(res, 112);
  }

  #[test]
  fn day17_second_actual() {
    let input = "target area: x=60..94, y=-171..-136";
    let res = super::main(input, true);
    assert_eq!(res, 2270);
  }
}
