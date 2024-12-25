use std::ops::{Add, Mul};

use crate::shared::grid::cartesian_product;

const MAX_PRESSES: u64 = 100;

#[derive(Debug)]
struct ClawMachine {
    button_a: Vec2,
    button_b: Vec2,
    prize: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2 {
    x: u64,
    y: u64,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<Vec2> for u64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut lines = input.lines();
    let mut machines = Vec::new();
    loop {
        let button_a_part = lines.next().unwrap().strip_prefix("Button A: ").unwrap();
        let button_a = parse_vec2(button_a_part);

        let button_b_part = lines.next().unwrap().strip_prefix("Button B: ").unwrap();
        let button_b = parse_vec2(button_b_part);

        let prize_part = lines.next().unwrap().strip_prefix("Prize: ").unwrap();
        let prize = parse_vec2(prize_part);

        machines.push(ClawMachine {
            button_a,
            button_b,
            prize,
        });

        if lines.next().is_none() {
            break;
        }
    }

    machines
}

fn parse_vec2(input: &str) -> Vec2 {
    let (x_part, y_part) = input.split_once(", ").unwrap();
    let x = x_part[2..].parse().unwrap();
    let y = y_part[2..].parse().unwrap();
    Vec2 { x, y }
}

pub fn part1(input: &str) -> u64 {
    let machines = parse_input(input);

    machines.iter().filter_map(tokens_to_win).sum()
}

fn tokens_to_win(machine: &ClawMachine) -> Option<u64> {
    cartesian_product(0..=MAX_PRESSES, 0..=MAX_PRESSES)
        .filter(|&(a, b)| a * machine.button_a + b * machine.button_b == machine.prize)
        .map(|(a, b)| 3 * a + b)
        .min()
}
