use std::collections::{BTreeMap, BTreeSet};

fn parse_input(input: &str) -> (i32, i32, BTreeMap<char, Vec<(i32, i32)>>) {
    let mut i = 0;
    let mut j = 0;
    let mut cols = 0;
    let mut frequencies = BTreeMap::new();
    for c in input.chars() {
        if c == '\n' {
            if cols == 0 {
                cols = j;
            } else {
                assert_eq!(j, cols);
            }
            i += 1;
            j = 0;
            continue;
        }

        if c != '.' {
            frequencies.entry(c).or_insert_with(Vec::new).push((i, j));
        }
        j += 1;
    }

    (i, cols, frequencies)
}

pub fn part1(input: &str) -> u64 {
    let (rows, cols, frequencies) = parse_input(input);

    let mut antinodes = BTreeSet::new();
    for towers in frequencies.values() {
        for m in 0..towers.len() {
            for n in 0..towers.len() {
                if m == n {
                    continue;
                }

                let (i1, j1) = towers[m];
                let (i2, j2) = towers[n];
                let i_antinode = 2 * i1 - i2;
                let j_antinode = 2 * j1 - j2;

                if i_antinode >= 0 && j_antinode >= 0 && i_antinode < rows && j_antinode < cols {
                    antinodes.insert((i_antinode, j_antinode));
                }
            }
        }
    }

    antinodes.len() as u64
}

pub fn part2(input: &str) -> u64 {
    let (rows, cols, frequencies) = parse_input(input);

    let mut antinodes = BTreeSet::new();
    for towers in frequencies.values() {
        for m in 0..towers.len() {
            for n in 0..towers.len() {
                if m == n {
                    continue;
                }

                let (i1, j1) = towers[m];
                let (i2, j2) = towers[n];
                let delta_i = i2 - i1;
                let delta_j = j2 - j1;
                let mut i = i1;
                let mut j = j1;

                while i >= 0 && j >= 0 && i < rows && j < cols {
                    antinodes.insert((i, j));

                    i += delta_i;
                    j += delta_j;
                }
            }
        }
    }

    antinodes.len() as u64
}
