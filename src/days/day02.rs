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
