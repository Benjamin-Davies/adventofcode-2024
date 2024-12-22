fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let rows = parse_input(input);

    rows.iter().filter(|row| is_safe(row)).count() as u64
}

fn is_safe(row: &[u64]) -> bool {
    (all_increasing(row) || all_decreasing(row)) && pairs_safe(row)
}

fn all_increasing(row: &[u64]) -> bool {
    row.windows(2).all(|pair| pair[0] < pair[1])
}

fn all_decreasing(row: &[u64]) -> bool {
    row.windows(2).all(|pair| pair[0] > pair[1])
}

fn pairs_safe(row: &[u64]) -> bool {
    row.windows(2).all(|pair| {
        let diff = u64::abs_diff(pair[0], pair[1]);
        diff >= 1 && diff <= 3
    })
}

pub fn part2(input: &str) -> u64 {
    let rows = parse_input(input);

    rows.iter()
        .filter(|row| is_safe_with_tolerance(row))
        .count() as u64
}

fn is_safe_with_tolerance(row: &[u64]) -> bool {
    if is_safe(row) {
        return true;
    }

    let mut skipped_row = Vec::with_capacity(row.len() - 1);
    for skip in 0..(row.len() - 1) {
        skipped_row.clear();
        skipped_row.extend(&row[..skip]);
        skipped_row.extend(&row[skip + 1..]);
        if is_safe(skipped_row.as_slice()) {
            return true;
        }
    }
    is_safe(&row[..row.len() - 1])
}
