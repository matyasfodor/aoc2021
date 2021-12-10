pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn big_computation() {
    alert("Big computation in Rust");
}

#[wasm_bindgen]
pub fn welcome(name: &str) {
    alert(&format!("Hello {}, from Rust!", name));
}

#[wasm_bindgen]
pub fn day01(input: &str, second: bool) -> usize {
    return day01::main(input, second);
}

#[wasm_bindgen]
pub fn day02(input: &str, second: bool) -> usize {
    return day02::main(input, second);
}

#[wasm_bindgen]
pub fn day03(input: &str, second: bool) -> usize {
    return day03::main(input, second);
}

#[wasm_bindgen]
pub fn day04(input: &str, second: bool) -> usize {
    return day04::main(input, second);
}

#[wasm_bindgen]
pub fn day05(input: &str, second: bool) -> usize {
    return day05::main(input, second);
}

#[wasm_bindgen]
pub fn day07(input: &str, second: bool) -> usize {
    return day07::main(input, second);
}

#[wasm_bindgen]
pub fn day08(input: &str, second: bool) -> usize {
    return day08::main(input, second);
}

#[wasm_bindgen]
pub fn day09(input: &str, second: bool) -> usize {
    return day09::main(input, second);
}

#[wasm_bindgen]
pub fn day10(input: &str, second: bool) -> usize {
    return day10::main(input, second);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
