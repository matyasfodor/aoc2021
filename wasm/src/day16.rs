use itertools::Itertools;
use std::vec::Vec;

struct Packet {
  version: usize,
  packet_type: usize,
  sub_packets: Vec<Packet>,
}

fn hex_to_binary(hex: &str) -> String {
  hex
    .chars()
    .map(|c| match c {
      '0' => "0000",
      '1' => "0001",
      '2' => "0010",
      '3' => "0011",
      '4' => "0100",
      '5' => "0101",
      '6' => "0110",
      '7' => "0111",
      '8' => "1000",
      '9' => "1001",
      'A' => "1010",
      'B' => "1011",
      'C' => "1100",
      'D' => "1101",
      'E' => "1110",
      'F' => "1111",
      _ => panic!("Unrecognised character"),
    })
    .collect()
}

fn str_to_bin(bin_idx: &str) -> usize {
  usize::from_str_radix(bin_idx, 2).expect("Expected a binary string")
}

fn parse_graph(s: &str) -> Packet {
  let binary = hex_to_binary(s);
  let mut binary_iter = binary.chars();
  let version_binary = binary_iter.take(3).collect::<String>();
  // let type_binary = binary_iter.take(3).collect::<String>();

  Packet {
    version: str_to_bin(&version_binary),
    packet_type: 0,
    sub_packets: vec![],
  }
}

fn get_sum_versions(graph: &Packet) -> usize {
  graph.version
}

pub fn main(s: &str, second: bool) -> usize {
  let graph = parse_graph(s);
  let sum_versions = get_sum_versions(&graph);
  sum_versions
}

#[cfg(test)]
mod tests {
  #[test]
  fn day16_first_simple() {
    let input = "D2FE28";
    let res = super::main(input, false);
    assert_eq!(res, 6);
  }

  #[test]
  fn day16_second() {
    let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    let res = super::main(input, true);
    assert_eq!(res, 5);
  }
}
