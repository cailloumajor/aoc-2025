use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

fn main() {
    let input_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "inputs", "day03.txt"]);
    let input = fs::read_to_string(input_path).expect("reading the input should not fail");

    // A mapping of the first found position by digit (ASCII).
    let mut digits_first_pos = BTreeMap::new();

    let mut total = 0u64;

    for line in input.lines() {
        digits_first_pos.clear();

        // Ensure all characters are non-zero ASCII digits.
        assert!(line.as_bytes().iter().all(|d| matches!(*d, b'1'..=b'9')));

        // Do not take the last digit for tens.
        let tens_digits = &line.as_bytes()[..line.len() - 1];
        for (pos, digit) in tens_digits.iter().enumerate() {
            digits_first_pos.entry(*digit).or_insert(pos);
        }
        let (tens_digit, tens_position) = digits_first_pos
            .last_key_value()
            .expect("a tens digit should have been found");

        let units_digit = line.as_bytes()[tens_position + 1..]
            .iter()
            .max()
            .expect("an units digit should have been found");

        let joltage = (tens_digit - b'0') * 10 + (units_digit - b'0');

        total += u64::from(joltage);
    }

    eprintln!("Total joltage is {total}");
}
