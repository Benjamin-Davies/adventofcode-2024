use std::collections::{btree_map::Entry, BTreeMap};

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

pub fn part1(input: &str) -> u64 {
    solution(input, 25)
}

pub fn part2(input: &str) -> u64 {
    solution(input, 75)
}

fn solution(input: &str, rounds: u64) -> u64 {
    let stones = parse_input(input);
    let mut frequencies = collect_frequencies(stones.iter().map(|&n| (n, 1)));

    for _ in 0..rounds {
        frequencies = blink(&frequencies);
    }
    frequencies.values().sum()
}

fn collect_frequencies(frequencies: impl Iterator<Item = (u64, u64)>) -> BTreeMap<u64, u64> {
    let mut result = BTreeMap::new();
    for (n, count) in frequencies {
        match result.entry(n) {
            Entry::Vacant(entry) => {
                entry.insert(count);
            }
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += count;
            }
        }
    }
    result
}

fn blink(frequencies: &BTreeMap<u64, u64>) -> BTreeMap<u64, u64> {
    collect_frequencies(
        frequencies
            .iter()
            .flat_map(|(&n, &count)| blink_single(n).map(move |m| (m, count))),
    )
}

fn blink_single(stone: u64) -> OneOrTwo {
    let digit_count = count_digits(stone);
    if stone == 0 {
        OneOrTwo::One(1)
    } else if digit_count % 2 == 0 {
        let divisor = 10u64.pow(digit_count / 2);
        OneOrTwo::Two(stone / divisor, stone % divisor)
    } else {
        OneOrTwo::One(stone * 2024)
    }
}

fn count_digits(n: u64) -> u32 {
    let mut count = 0;
    let mut power = 1;
    while power <= n {
        power *= 10;
        count += 1;
    }
    count
}

#[derive(Debug, Clone, Copy)]
enum OneOrTwo {
    Zero,
    One(u64),
    Two(u64, u64),
}

impl Iterator for OneOrTwo {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            OneOrTwo::Zero => None,
            OneOrTwo::One(x) => {
                *self = OneOrTwo::Zero;
                Some(x)
            }
            OneOrTwo::Two(x, y) => {
                *self = OneOrTwo::One(y);
                Some(x)
            }
        }
    }
}
