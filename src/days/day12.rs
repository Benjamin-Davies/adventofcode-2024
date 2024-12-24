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
    let mut queue = VecDeque::new();
    for (i_start, j_start) in map.positions() {
        if visited[i_start as usize][j_start as usize] {
            continue;
        }

        queue.push_back((i_start as isize, j_start as isize));
        let mut area = 0;
        let mut perimeter = 0;
        while let Some((i, j)) = queue.pop_back() {
            if visited[i as usize][j as usize] {
                continue;
            }
            visited[i as usize][j as usize] = true;
            let c = map[i as usize][j as usize];
            area += 1;

            for (i_dir, j_dir) in DIRECTIONS {
                let i_next = i + i_dir;
                let j_next = j + j_dir;
                if map.contains(i_next, j_next) && map[i_next as usize][j_next as usize] == c {
                    queue.push_back((i_next, j_next));
                } else {
                    perimeter += 1;
                }
            }
        }

        total_price += area * perimeter;
    }

    total_price
}
