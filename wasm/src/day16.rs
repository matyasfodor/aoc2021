use itertools::Itertools;
// use std::vec::Vec;
use take_until::TakeUntilExt;

struct Packet {
  packet_version: usize,
  packet_type: usize,
  sub_packets: Vec<Packet>,
  value: Option<usize>,
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

fn parse_number(raw: &str) -> (usize, usize) {
  // let chunks = ;
  let binary_string: String = raw
    .chars()
    .tuples::<(_, _, _, _, _)>()
    .take_until(|(first_char, _, _, _, _)| *first_char == '0')
    .flat_map(|coding_digits| {
      [
        coding_digits.1,
        coding_digits.2,
        coding_digits.3,
        coding_digits.4,
      ]
    })
    .collect();
  (str_to_bin(&binary_string), (binary_string.len() / 4) * 5)
}

fn parse_graph(s: &str) -> (Packet, usize) {
  let binary = hex_to_binary(s);

  let version_binary = binary.chars().take(3).collect::<String>();
  let packet_version = str_to_bin(&version_binary);

  let type_binary = binary.chars().skip(3).take(3).collect::<String>();
  let packet_type = str_to_bin(&type_binary);

  if packet_type == 4 {
    let rest: String = binary.chars().skip(6).collect();

    let (number, consumed_bits) = parse_number(&rest);
    (
      Packet {
        packet_version,
        packet_type,
        // TODO use enum
        sub_packets: vec![],
        value: Some(number),
      },
      6 + consumed_bits,
    )
    // read number
  } else {
    // let length_type = binary.chars().skip(6).next().expect("Should be a char");
    // operator, read subpackets
    panic!("Not implemented")
    // if length_type == '1' {
    //   let raw_subpackets = binary.chars().skip(7).take(11).collect();
    //   let num_subpackets = parse_number(&raw_subpackets);
    //   let subpackets = (0..num_subpackets).map(|_| {})
    //   // the next 11 bits are a number that represents the number of sub-packets immediately contained by this packet
    // } else {
    //   // the next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet

    // }
  }
}

fn get_sum_versions(graph: &Packet) -> usize {
  graph.packet_version
}

pub fn main(s: &str, _second: bool) -> usize {
  let (graph, _) = parse_graph(s);
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
  fn parse_number_test() {
    let (value, consumed_length) = super::parse_number("101111111000101000");
    assert_eq!(value, 2021);
    assert_eq!(consumed_length, 15);
  }

  #[test]
  fn day16_first_operator_first() {
    let input = "38006F45291200";
    let res = super::main(input, false);
    assert_eq!(res, 1);
  }

  #[test]
  fn day16_second() {
    let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    let res = super::main(input, true);
    assert_eq!(res, 5);
  }
}
