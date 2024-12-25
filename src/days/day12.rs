use std::collections::VecDeque;

use crate::shared::grid::Grid;

fn parse_input(input: &str) -> Grid<char> {
    let mut map = Grid::builder();
    for c in input.chars() {
        if c == '\n' {
            map.newline();
        } else {
            map.push(c);
        }
    }
    map.build()
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn part1(input: &str) -> u64 {
    let map = parse_input(input);

    let mut total_price = 0;
    let mut visited = Grid::new(map.rows(), map.cols(), false);
    let mut fence_visited = Grid::new(map.rows(), map.cols(), 0);
    let mut queue = VecDeque::new();
    for (i, j) in map.positions() {
        if visited[i as usize][j as usize] {
            continue;
        }

        let (area, perimeter, _fences) =
            find_area_and_perimeter(i, j, &map, &mut visited, &mut fence_visited, &mut queue);

        total_price += area * perimeter;
    }

    total_price
}

pub fn part2(input: &str) -> u64 {
    let map = parse_input(input);

    let mut total_price = 0;
    let mut visited = Grid::new(map.rows(), map.cols(), false);
    let mut fence_visited = Grid::new(map.rows(), map.cols(), 0);
    let mut queue = VecDeque::new();
    for (i, j) in map.positions() {
        if visited[i as usize][j as usize] {
            continue;
        }

        let (area, _perimeter, fences) =
            find_area_and_perimeter(i, j, &map, &mut visited, &mut fence_visited, &mut queue);

        total_price += area * fences;
    }

    total_price
}

fn find_area_and_perimeter(
    i_start: usize,
    j_start: usize,
    map: &Grid<char>,
    visited: &mut Grid<bool>,
    fence_visited: &mut Grid<u8>,
    queue: &mut VecDeque<(isize, isize)>,
) -> (u64, u64, u64) {
    let c = map[i_start][j_start];
    queue.push_back((i_start as isize, j_start as isize));

    let mut area = 0;
    let mut perimeter = 0;
    let mut fences = 0;
    while let Some((i, j)) = queue.pop_back() {
        if visited[i as usize][j as usize] {
            continue;
        }
        visited[i as usize][j as usize] = true;
        area += 1;

        for (i_dir, j_dir) in DIRECTIONS {
            let i_next = i + i_dir;
            let j_next = j + j_dir;
            if map.contains(i_next, j_next) && map[i_next as usize][j_next as usize] == c {
                queue.push_back((i_next, j_next));
            } else {
                perimeter += 1;
                if mark_fence(i, j, i_dir, j_dir, map, fence_visited) {
                    fences += 1;
                }
            }
        }
    }

    (area, perimeter, fences)
}

/// Marks a fence as visited.
///
/// Returns true if the fence has not already been marked as visited.
fn mark_fence(
    i_start: isize,
    j_start: isize,
    i_dir: isize,
    j_dir: isize,
    map: &Grid<char>,
    fence_visited: &mut Grid<u8>,
) -> bool {
    let c = map[i_start as usize][j_start as usize];
    let dir_bitmask = match (i_dir, j_dir) {
        (1, 0) => 1 << 0,
        (0, 1) => 1 << 1,
        (-1, 0) => 1 << 2,
        (0, -1) => 1 << 3,
        _ => unreachable!(),
    };
    if (fence_visited[i_start as usize][j_start as usize] & dir_bitmask) != 0 {
        return false;
    }
    fence_visited[i_start as usize][j_start as usize] |= dir_bitmask;

    let inside = |i, j| map.contains(i, j) && map[i as usize][j as usize] == c;
    let on_fence = |i, j| inside(i, j) && !inside(i + i_dir, j + j_dir);

    let mut i = i_start;
    let mut j = j_start;
    'left: loop {
        i -= j_dir;
        j += i_dir;
        if on_fence(i, j) {
            fence_visited[i as usize][j as usize] |= dir_bitmask;
            continue 'left;
        } else {
            break;
        }
    }

    let mut i = i_start;
    let mut j = j_start;
    'right: loop {
        i += j_dir;
        j -= i_dir;
        if on_fence(i, j) {
            fence_visited[i as usize][j as usize] |= dir_bitmask;
            continue 'right;
        } else {
            break;
        }
    }

    true
}
