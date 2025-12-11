use std::fs;
use std::path::PathBuf;

fn main() {
    let input_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "inputs", "day02.txt"]);
    let input = fs::read_to_string(input_path).expect("reading the input should not fail");

    let mut total = 0u64;

    for range_str in input.trim().split(',') {
        let (min_str, max_str) = range_str
            .split_once('-')
            .expect("splitting range string should not fail");
        let [min, max] = [min_str, max_str].map(|s| {
            s.parse::<u64>()
                .expect("parsing range boundary should not fail")
        });

        for num in min..=max {
            let num_str = num.to_string();
            let num_len = num_str.len();
            if let (half_len, 0) = (num_len / 2, num_len % 2)
                && let (a, b) = num_str.split_at(half_len)
                && a == b
            {
                total += num;
            }
        }
    }

    eprintln!("The sum of invalid IDs is: {total}");
}
