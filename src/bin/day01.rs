use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

const DIAL_START: u8 = 50;
const DIAL_MAX: i64 = 100;

fn main() {
    let input_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "inputs", "day01.txt"]);
    let input = File::open(input_path).expect("opening the input file should not fail");
    let input = BufReader::new(input);

    let mut dial_position = DIAL_START;
    let mut zero_stop_count: u32 = 0;
    let mut zero_pass_count: i64 = 0;

    for line_result in input.lines() {
        let mut line = line_result.expect("reading input line should not fail");
        let direction = line.remove(0);
        let quantity: i64 = line
            .parse()
            .expect("parsing the tick quantity qhould not fail");

        let new_raw_position = match direction {
            'L' => i64::from(dial_position) - quantity,
            'R' => i64::from(dial_position) + quantity,
            _ => panic!("unexpected dial quantity"),
        };

        if new_raw_position <= 0 {
            if dial_position > 0 {
                zero_pass_count += 1;
            }
            zero_pass_count += new_raw_position / -DIAL_MAX;
        } else {
            zero_pass_count += new_raw_position / DIAL_MAX;
        }

        dial_position = new_raw_position
            .rem_euclid(DIAL_MAX)
            .try_into()
            .expect("dial position should fit in an u8");

        if dial_position == 0 {
            zero_stop_count += 1;
        }
    }

    eprintln!("Zero was encountered {zero_stop_count} times");
    eprintln!("Zero was passed through {zero_pass_count} times");
}
