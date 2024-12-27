use std::{cmp::Reverse, collections::BinaryHeap};

use crate::shared::{grid::Grid, vec2::Vec2};

fn parse_input(input: &str) -> Vec<Vec2> {
    input
        .lines()
        .map(|line| {
            let (x_part, y_part) = line.split_once(',').unwrap();
            Vec2 {
                x: x_part.parse().unwrap(),
                y: y_part.parse().unwrap(),
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Occupied,
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::Occupied => '#',
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let incoming = parse_input(input);

    let start = Vec2 { x: 0, y: 0 };
    let end = Vec2 {
        x: incoming.iter().map(|v| v.x).max().unwrap(),
        y: incoming.iter().map(|v| v.y).max().unwrap(),
    };
    let w = end.x as usize + 1;
    let h = end.y as usize + 1;

    let n = if end.x <= 6 { 12 } else { 1024 };
    let mut map = Grid::new(h, w, Tile::Empty);
    for &pos in &incoming[..n] {
        map[pos] = Tile::Occupied;
    }

    let mut visited = Grid::new(h, w, false);
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), start));
    while let Some((Reverse(score), pos)) = heap.pop() {
        if pos == end {
            return score;
        }

        if visited[pos] || map[pos] == Tile::Occupied {
            continue;
        }
        visited[pos] = true;

        for dir in Vec2::CARDINAL_DIRECTIONS {
            let new_pos = pos + dir;
            if map.contains(new_pos.y as isize, new_pos.x as isize) {
                heap.push((Reverse(score + 1), new_pos));
            }
        }
    }

    unimplemented!()
}
