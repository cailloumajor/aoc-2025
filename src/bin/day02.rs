use std::fs;
use std::path::PathBuf;

fn main() {
    let input_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "inputs", "day02.txt"]);
    let input = fs::read_to_string(input_path).expect("reading the input should not fail");

    let mut total_part1 = 0u64;
    let mut total_part2 = 0u64;

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
            let num_ascii = num_str.as_bytes();
            let num_len = num_ascii.len();

            if let (half_len, 0) = (num_len / 2, num_len % 2)
                && let (a, b) = num_ascii.split_at(half_len)
                && a == b
            {
                total_part1 += num;
            }

            for s in 1..=(num_len / 2) {
                let mut chunks = num_ascii.chunks_exact(s);
                if !chunks.remainder().is_empty() {
                    continue;
                }
                let Some(first_chunk) = chunks.next() else {
                    continue;
                };
                if chunks.all(|c| c == first_chunk) {
                    total_part2 += num;

                    // Prevent checking the same number more than one time.
                    break;
                }
            }
        }
    }

    eprintln!("The sum of invalid IDs for part 1 is: {total_part1}");
    eprintln!("The sum of invalid IDs for part 2 is: {total_part2}");
}
