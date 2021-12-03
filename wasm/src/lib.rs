pub mod day01;
pub mod day02;
pub mod day03;
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
