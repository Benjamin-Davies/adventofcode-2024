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
    BestPath,
}

impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        match value {
            Cell::Empty => '.',
            Cell::Wall => '#',
            Cell::Start => 'S',
            Cell::End => 'E',
            Cell::BestPath => 'O',
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

pub fn part2(input: &str) -> u64 {
    let (mut map, start, end) = parse_input(input);

    // Dijkstra
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), start, Vec2::EAST));
    let mut visited = Grid::new(map.cols(), map.rows(), [u64::MAX; 4]);
    while let Some((Reverse(score), pos, dir)) = heap.pop() {
        if map[pos] == Cell::Wall {
            continue;
        }

        let dir_ordinal = dir.dir_ordinal() as usize;
        if visited[pos][dir_ordinal] <= score {
            continue;
        }
        visited[pos][dir_ordinal] = score;

        if pos == end {
            break;
        }

        heap.push((Reverse(score + FORWARD_WEIGHT), pos + dir, dir));
        heap.push((Reverse(score + ROTATE_WEIGHT), pos, dir.rotate_cw()));
        heap.push((Reverse(score + ROTATE_WEIGHT), pos, dir.rotate_ccw()));
    }
    while let Some((Reverse(score), pos, dir)) = heap.pop() {
        let dir_ordinal = dir.dir_ordinal() as usize;
        if visited[pos][dir_ordinal] <= score {
            continue;
        }
        visited[pos][dir_ordinal] = score;
    }
    let best_score = visited[end].into_iter().min().unwrap();

    // Backtrack
    let mut stack = Vec::new();
    for (end_score, end_dir) in visited[end].into_iter().zip(Vec2::CARDINAL_DIRECTIONS) {
        if end_score == best_score {
            stack.push((end, end_dir));
        }
    }
    while let Some((pos, dir)) = stack.pop() {
        map[pos] = Cell::BestPath;

        if pos == start {
            continue;
        }

        let score = visited[pos][dir.dir_ordinal() as usize];
        if visited[pos - dir][dir.dir_ordinal() as usize].saturating_add(FORWARD_WEIGHT) == score {
            stack.push((pos - dir, dir));
        }
        if visited[pos][dir.rotate_cw().dir_ordinal() as usize].saturating_add(ROTATE_WEIGHT)
            == score
        {
            stack.push((pos, dir.rotate_cw()));
        }
        if visited[pos][dir.rotate_ccw().dir_ordinal() as usize].saturating_add(ROTATE_WEIGHT)
            == score
        {
            stack.push((pos, dir.rotate_ccw()));
        }
    }

    map.positions()
        .filter(|&(i, j)| map[i][j] == Cell::BestPath)
        .count() as u64
}
