use std::iter;
use std::str::FromStr;

fn main() {
    let input = include_str!("../../inputs/day06.txt");

    let mut input_lines = input.lines().peekable();

    let numbers_lines: Vec<Vec<u64>> =
        iter::from_fn(|| input_lines.next_if(|l| !l.trim_start().starts_with(['+', '*'])))
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(u64::from_str)
                    .collect::<Result<_, _>>()
                    .expect("parsing number should not fail")
            })
            .collect();

    let total: u64 = input_lines
        .next()
        .expect("line of operation signs should exist")
        .split_ascii_whitespace()
        .enumerate()
        .map(|(i, op)| {
            let col = numbers_lines.iter().map(|l| l[i]);
            match op {
                "+" => col.sum::<u64>(),
                "*" => col.product::<u64>(),
                _ => panic!("invalid operation sign: '{op}'"),
            }
        })
        .sum();

    eprintln!("The total of all operations is {total}");
}
