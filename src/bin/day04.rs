use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Relative position to adjacent cells.
const ADJACENT_POSITIONS: [(isize, isize); 8] = [
    (-1, -1), // Top-left
    (0, -1),  // Top
    (1, -1),  // Top-right
    (-1, 0),  // Left
    (1, 0),   // Right
    (-1, 1),  // Bottom-left
    (0, 1),   // Bottom
    (1, 1),   // Bottom-right
];

fn main() {
    let input_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "inputs", "day04.txt"]);
    let input_file = File::open(input_path).expect("opening the input should not fail");
    let input = BufReader::new(input_file);

    let mut accessible = 0u64;

    let mut matrix = Vec::new();

    for l in input.lines() {
        let line = l.expect("reading a line should not fail");
        let line_bool = line
            .into_bytes()
            .into_iter()
            .map(|b| match b {
                b'@' => true,
                b'.' => false,
                _ => panic!("invalid character: {}", char::from(b)),
            })
            .collect::<Vec<_>>();
        matrix.push(line_bool);
    }

    for (y, line) in matrix.iter().enumerate() {
        for (x, column) in line.iter().enumerate() {
            if !*column {
                // No roll here, continue.
                continue;
            }
            let occupied_adjacent = ADJACENT_POSITIONS
                .iter()
                .filter(|(delta_x, delta_y)| {
                    let Some((adj_x, adj_y)) = x
                        .checked_add_signed(*delta_x)
                        .zip(y.checked_add_signed(*delta_y))
                    else {
                        return false;
                    };
                    matrix
                        .get(adj_y)
                        .and_then(|line| line.get(adj_x))
                        .copied()
                        .unwrap_or(false)
                })
                .count();
            if occupied_adjacent < 4 {
                accessible += 1;
            }
        }
    }

    eprintln!("There are {accessible} accessible rolls");
}
