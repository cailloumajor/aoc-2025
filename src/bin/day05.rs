fn main() {
    let input = include_str!("../../inputs/day05.txt");

    let mut lines = input.lines();

    let mut ranges = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|line| {
            let (start_str, end_str) = line
                .split_once('-')
                .expect("splitting range line should not fail");
            let start = start_str
                .parse::<u64>()
                .expect("parsing start bound should not fail");
            let end = end_str
                .parse::<u64>()
                .expect("parsing end bound should not fail");
            (start, end)
        })
        .collect::<Vec<_>>();

    let fresh_count = lines
        .map(|line| line.parse::<u64>().expect("parsing ID should not fail"))
        .filter(|id| {
            ranges
                .iter()
                .any(|(start, end)| (start..=end).contains(&id))
        })
        .count();

    eprintln!("There are {fresh_count} fresh ingredients IDs");

    ranges.sort();

    let unmerged_ranges = ranges.split_off(1);
    for r in unmerged_ranges {
        let last_merged = ranges
            .last_mut()
            .expect("ranges vector should not be empty");
        if r.0 > last_merged.1 {
            ranges.push(r);
        } else if r.1 > last_merged.1 {
            last_merged.1 = r.1
        }
    }
    let fresh_ids_count: u64 = ranges.into_iter().map(|(start, end)| end - start + 1).sum();

    eprintln!("There are {fresh_ids_count} fresh IDs in the ranges");
}
