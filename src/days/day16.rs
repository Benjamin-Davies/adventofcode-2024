use std::{cmp::Reverse, collections::BinaryHeap};

use crate::shared::{grid::Grid, vec2::Vec2};

const FORWARD_WEIGHT: u64 = 1;
const ROTATE_WEIGHT: u64 = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Start,
    End,
}

impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        match value {
            Cell::Empty => '.',
            Cell::Wall => '#',
            Cell::Start => 'S',
            Cell::End => 'E',
        }
    }
}

fn parse_input(input: &str) -> (Grid<Cell>, Vec2, Vec2) {
    let mut map = Grid::builder();
    let mut start = None;
    let mut end = None;
    for c in input.chars() {
        match c {
            '\n' => map.newline(),
            '.' => map.push(Cell::Empty),
            '#' => map.push(Cell::Wall),
            'S' => {
                start = Some(map.next_position_vec2());
                map.push(Cell::Start);
            }
            'E' => {
                end = Some(map.next_position_vec2());
                map.push(Cell::End);
            }
            _ => unimplemented!(),
        }
    }
    (map.build(), start.unwrap(), end.unwrap())
}

pub fn part1(input: &str) -> u64 {
    let (map, start, end) = parse_input(input);

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), start, Vec2::EAST));
    let mut visited = Grid::new(map.cols(), map.rows(), 0);
    while let Some((Reverse(score), pos, dir)) = heap.pop() {
        if pos == end {
            return score;
        }
        if map[pos] == Cell::Wall {
            continue;
        }

        let dir_bitmask = 1u8 << dir.dir_ordinal();
        if (visited[pos] & dir_bitmask) != 0 {
            continue;
        }
        visited[pos] |= dir_bitmask;

        heap.push((Reverse(score + FORWARD_WEIGHT), pos + dir, dir));
        heap.push((Reverse(score + ROTATE_WEIGHT), pos, dir.rotate_cw()));
        heap.push((Reverse(score + ROTATE_WEIGHT), pos, dir.rotate_ccw()));
    }

    unimplemented!()
}
