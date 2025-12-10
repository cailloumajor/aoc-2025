use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

const DIAL_START: i64 = 50;
const DIAL_MAX: i64 = 100;

fn main() {
    let input_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "inputs", "day01.txt"]);
    let input = File::open(input_path).expect("opening the input file should not fail");
    let input = BufReader::new(input);

    let mut dial_position = DIAL_START;
    let mut zero_count: u32 = 0;

    for line_result in input.lines() {
        let mut line = line_result.expect("reading input line should not fail");
        let direction = line.remove(0);
        let quantity: i64 = line
            .parse()
            .expect("parsing the tick quantity qhould not fail");
        match direction {
            'L' => {
                dial_position = (dial_position - quantity) % DIAL_MAX;
            }
            'R' => {
                dial_position = (dial_position + quantity) % DIAL_MAX;
            }
            _ => panic!("unexpected dial quantity"),
        }
        if dial_position == 0 {
            zero_count += 1;
        }
    }

    eprintln!("Zero was encountered {zero_count} times");
}
