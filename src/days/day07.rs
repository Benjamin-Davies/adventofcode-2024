fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (test_value, operands) = line.split_once(": ").unwrap();
            (
                test_value.parse().unwrap(),
                operands
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let equations = parse_input(input);

    equations
        .into_iter()
        .filter(|(test_value, operands)| could_be_true(*test_value, operands))
        .map(|(test_value, _)| test_value)
        .sum()
}

fn could_be_true(test_value: u64, operands: &[u64]) -> bool {
    let operand_count = operands.len() as u32;
    let operator_count = operand_count - 1;
    assert!(operator_count < u64::BITS);

    'outer: for combination in 0..(1 << operator_count) {
        let mut result = operands[0];
        for i in 0..operator_count {
            if (combination & 1 << i) == 0 {
                result += operands[i as usize + 1];
            } else {
                result *= operands[i as usize + 1];
            }

            if result > test_value {
                continue 'outer;
            }
        }

        if result == test_value {
            return true;
        }
    }

    false
}

pub fn part2(input: &str) -> u64 {
    let equations = parse_input(input);

    equations
        .into_iter()
        .filter(|(test_value, operands)| could_be_true_part2(*test_value, operands))
        .map(|(test_value, _)| test_value)
        .sum()
}

fn could_be_true_part2(test_value: u64, operands: &[u64]) -> bool {
    let operand_count = operands.len() as u32;
    let operator_count = operand_count - 1;
    let total_combinations = 3u64.pow(operand_count);

    'outer: for combination in 0..total_combinations {
        let mut result = operands[0];
        let mut divisor = 1;
        for i in 0..operator_count {
            match (combination / divisor) % 3 {
                0 => {
                    result += operands[i as usize + 1];
                }
                1 => {
                    result *= operands[i as usize + 1];
                }
                2 => {
                    result = concat_uint(result, operands[i as usize + 1]);
                }
                _ => unreachable!(),
            }

            if result > test_value {
                continue 'outer;
            }

            divisor *= 3;
        }

        if result == test_value {
            return true;
        }
    }

    false
}

fn concat_uint(x: u64, y: u64) -> u64 {
    let mut multiplier = 1;
    while multiplier <= y {
        multiplier *= 10;
    }
    x * multiplier + y
}
