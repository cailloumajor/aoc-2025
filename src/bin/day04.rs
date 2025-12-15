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

    let mut removed = Vec::new();

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

    while let r = accessible_rolls(&matrix)
        && !r.is_empty()
    {
        removed.push(r.len());

        for (x, y) in r {
            matrix[y][x] = false;
        }
    }

    let removed_first = removed[0];
    eprintln!("There are {removed_first} accessible rolls in the first part");

    let total_removed: usize = removed.into_iter().sum();
    eprintln!("There were {total_removed} rolls removed");
}

fn accessible_rolls<L: AsRef<[bool]>>(matrix: &[L]) -> Vec<(usize, usize)> {
    let mut coordinates = Vec::new();

    for (y, line) in matrix.iter().enumerate() {
        for x in line
            .as_ref()
            .iter()
            .enumerate()
            .filter_map(|(x, occupied)| occupied.then_some(x))
        {
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
                        .and_then(|line| line.as_ref().get(adj_x))
                        .copied()
                        .unwrap_or(false)
                })
                .count();
            if occupied_adjacent < 4 {
                coordinates.push((x, y));
            }
        }
    }

    coordinates
}
