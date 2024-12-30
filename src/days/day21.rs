use std::{collections::BTreeMap, iter, sync::Mutex};

use crate::shared::vec2::Vec2;

fn parse_input(input: &str) -> Vec<(&str, u64)> {
    input
        .lines()
        .map(|line| (line, line.trim_end_matches('A').parse().unwrap()))
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let codes = parse_input(input);

    codes
        .iter()
        .map(|&(code, n)| shortest_code_all(code, 3) * n)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let codes = parse_input(input);

    codes
        .iter()
        .map(|&(code, n)| shortest_code_all(code, 26) * n)
        .sum()
}

fn shortest_code_all(code: &str, levels: u64) -> u64 {
    let buttons = code.chars().map(numeric_keypad).collect::<Vec<_>>();

    shortest_code(&buttons, Vec2::ZERO, levels)
}

fn shortest_code(buttons: &[Vec2], start_pos: Vec2, level: u64) -> u64 {
    if buttons.is_empty() {
        return 0;
    }
    if level == 0 {
        return buttons.len() as u64;
    }

    let mut total = 0;
    let mut last_pos = start_pos;
    for &button in buttons {
        let p1 = last_pos;
        let p2 = button;

        let min = shortest_code_single(p1, p2, level);

        total += min;
        last_pos = button;
    }

    total
}

fn shortest_code_single(p1: Vec2, p2: Vec2, level: u64) -> u64 {
    static CACHE: Mutex<BTreeMap<(Vec2, Vec2, u64), u64>> = Mutex::new(BTreeMap::new());

    if let Some(&min) = CACHE.lock().unwrap().get(&(p1, p2, level)) {
        return min;
    }

    let mut combinations = button_combinations(p1, p2);
    for next_buttons in &mut combinations {
        next_buttons.push(Vec2::ZERO);
    }
    let min = combinations
        .iter()
        .map(|c| shortest_code(c, Vec2::ZERO, level - 1))
        .min()
        .unwrap();

    CACHE.lock().unwrap().insert((p1, p2, level), min);

    min
}

fn button_combinations(p1: Vec2, p2: Vec2) -> Vec<Vec<Vec2>> {
    let delta_x = p1.x.abs_diff(p2.x) as usize;
    let delta_y = p1.y.abs_diff(p2.y) as usize;
    if p1 == p2 {
        return vec![vec![]];
    } else if p1.x == p2.x {
        let dir = if p1.y < p2.y { 'v' } else { '^' };
        return vec![vec![directional_keypad(dir); delta_y]];
    } else if p1.y == p2.y {
        let dir = if p1.x < p2.x { '>' } else { '<' };
        return vec![vec![directional_keypad(dir); delta_x]];
    } else {
        let x_dir = if p1.x < p2.x { '>' } else { '<' };
        let y_dir = if p1.y < p2.y { 'v' } else { '^' };
        let x_moves = iter::repeat(directional_keypad(x_dir)).take(delta_x);
        let y_moves = iter::repeat(directional_keypad(y_dir)).take(delta_y);

        let mut combos = Vec::new();
        // Avoid going through (-2, 0), which is always the empty tile
        if p1.y != 0 || p2.x != -2 {
            let combo1 = x_moves.clone().chain(y_moves.clone()).collect();
            combos.push(combo1);
        }
        if p1.x != -2 || p2.y != 0 {
            let combo2 = y_moves.clone().chain(x_moves.clone()).collect();
            combos.push(combo2);
        }

        return combos;
    }
}

fn numeric_keypad(c: char) -> Vec2 {
    match c {
        'A' => Vec2 { x: 0, y: 0 },
        '0' => Vec2 { x: -1, y: 0 },
        '1' => Vec2 { x: -2, y: -1 },
        '2' => Vec2 { x: -1, y: -1 },
        '3' => Vec2 { x: 0, y: -1 },
        '4' => Vec2 { x: -2, y: -2 },
        '5' => Vec2 { x: -1, y: -2 },
        '6' => Vec2 { x: 0, y: -2 },
        '7' => Vec2 { x: -2, y: -3 },
        '8' => Vec2 { x: -1, y: -3 },
        '9' => Vec2 { x: 0, y: -3 },
        _ => unimplemented!("{c}"),
    }
}

fn directional_keypad(c: char) -> Vec2 {
    match c {
        'A' => Vec2 { x: 0, y: 0 },
        '^' => Vec2 { x: -1, y: 0 },
        '<' => Vec2 { x: -2, y: 1 },
        'v' => Vec2 { x: -1, y: 1 },
        '>' => Vec2 { x: 0, y: 1 },
        _ => unimplemented!("{c}"),
    }
}
