use crate::shared::grid::Grid;

const IMPASSIBLE: u8 = u8::MAX;

fn parse_input(input: &str) -> Grid<u8> {
    let mut builder = Grid::builder();
    for c in input.chars() {
        match c {
            '\n' => builder.newline(),
            '0'..='9' => builder.push(c as u8 - b'0'),
            _ => builder.push(IMPASSIBLE),
        }
    }
    builder.build()
}

pub fn part1(input: &str) -> u64 {
    let map = parse_input(input);

    let mut total_score = 0;
    let mut visited = Vec::new();
    let mut summits = Vec::new();
    for (i, j) in map.positions() {
        if map[i][j] == 0 {
            traverse(&map, i as isize, j as isize, &mut visited, &mut summits);

            total_score += summits.len() as u64;
            visited.clear();
            summits.clear();
        }
    }
    total_score
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn traverse(
    map: &Grid<u8>,
    i: isize,
    j: isize,
    visited: &mut Vec<(isize, isize)>,
    summits: &mut Vec<(isize, isize)>,
) {
    if visited.contains(&(i, j)) {
        return;
    }
    visited.push((i, j));

    let value = map[i as usize][j as usize];
    if value == 9 {
        summits.push((i, j));
        return;
    }

    for (i_dir, j_dir) in DIRECTIONS {
        let i2 = i + i_dir;
        let j2 = j + j_dir;
        if !map.contains(i2, j2) {
            continue;
        }
        let value2 = map[i2 as usize][j2 as usize];
        if value2 == value + 1 {
            traverse(map, i2, j2, visited, summits);
        }
    }
}
