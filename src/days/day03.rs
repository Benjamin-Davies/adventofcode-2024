use regex::Regex;

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|caps| (caps[1].parse().unwrap(), caps[2].parse().unwrap()))
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let products = parse_input(input);

    products.into_iter().map(|(a, b)| a * b).sum()
}

#[derive(Debug)]
enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
}

fn parse_input_part2(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    re.captures_iter(input)
        .map(|caps| match &caps[0][..3] {
            "mul" => Instruction::Mul(caps[1].parse().unwrap(), caps[2].parse().unwrap()),
            "do(" => Instruction::Do,
            "don" => Instruction::Dont,
            _ => unimplemented!(),
        })
        .collect()
}

pub fn part2(input: &str) -> u64 {
    let instructions = parse_input_part2(input);

    let mut enabled = true;
    let mut sum = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Mul(a, b) => {
                if enabled {
                    sum += a * b;
                }
            }
            Instruction::Do => {
                enabled = true;
            }
            Instruction::Dont => {
                enabled = false;
            }
        }
    }
    sum
}
