fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let mut stones = parse_input(input);

    for _ in 0..25 {
        stones = blink(&stones);
    }
    stones.len() as u64
}

fn blink(stones: &[u64]) -> Vec<u64> {
    let mut new_stones = Vec::with_capacity(stones.len());
    for &stone in stones {
        let digit_count = count_digits(stone);
        if stone == 0 {
            new_stones.push(1);
        } else if digit_count % 2 == 0 {
            let divisor = 10u64.pow(digit_count / 2);
            new_stones.push(stone / divisor);
            new_stones.push(stone % divisor);
        } else {
            new_stones.push(stone * 2024);
        }
    }
    new_stones
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
