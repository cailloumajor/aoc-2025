use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

fn main() {
    let input_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "inputs", "day03.txt"]);
    let input = fs::read_to_string(input_path).expect("reading the input should not fail");

    let total_part1: u64 = input
        .lines()
        .map(|l| max_bank_joltage::<2>(l.as_bytes()))
        .sum();

    eprintln!("Total joltage for part 1 is {total_part1}");

    let total_part2: u64 = input
        .lines()
        .map(|l| max_bank_joltage::<12>(l.as_bytes()))
        .sum();

    eprintln!("Total joltage for part 2 is {total_part2}");
}

/// Process the maximum joltage of a battery bank, given the number of batteries to enable in the bank.
fn max_bank_joltage<const BAT_COUNT: usize>(bank: &[u8]) -> u64 {
    let mut joltage_bytes = [0u8; BAT_COUNT];

    // A mapping of the first found position by digit (ASCII).
    let mut digits_first_pos = BTreeMap::new();

    let mut start_bank_index = 0usize;

    for (idx, elt) in joltage_bytes.iter_mut().enumerate() {
        digits_first_pos.clear();

        let end_index = bank.len() - BAT_COUNT + idx;

        for (pos, digit) in bank
            .iter()
            .enumerate()
            .skip(start_bank_index)
            .take_while(|(i, _)| *i <= end_index)
        {
            digits_first_pos.entry(*digit).or_insert(pos);
        }
        let (max_digit, position) = digits_first_pos
            .last_key_value()
            .expect("a digit should have been found");
        *elt = *max_digit;
        start_bank_index = *position + 1;
    }

    str::from_utf8(&joltage_bytes)
        .expect("joltage value should be valid utf-8")
        .parse()
        .expect("parsing joltage value should not fail")
}
