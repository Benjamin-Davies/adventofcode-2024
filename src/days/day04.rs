use crate::shared::grid::Grid;

const WORD: &str = "XMAS";
const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn parse_input(input: &str) -> Grid<u8> {
    let mut builder = Grid::builder();
    for c in input.chars() {
        if c == '\n' {
            builder.newline();
        } else {
            let i = WORD.find(c).map(|i| i as u8).unwrap_or(u8::MAX);
            builder.push(i);
        }
    }
    builder.build()
}

pub fn part1(input: &str) -> u64 {
    let grid = parse_input(input);

    let mut count = 0;
    for (i_start, j_start) in grid.positions() {
        for (i_dir, j_dir) in DIRECTIONS {
            let i_end = i_start as isize + (WORD.len() as isize - 1) * i_dir;
            let j_end = j_start as isize + (WORD.len() as isize - 1) * j_dir;
            if !grid.contains(i_end, j_end) {
                continue;
            }

            if (0..WORD.len()).all(|k| {
                grid[(i_start as isize + k as isize * i_dir) as usize]
                    [(j_start as isize + k as isize * j_dir) as usize]
                    == k as u8
            }) {
                count += 1;
            }
        }
    }
    count
}
pub fn part2(input: &str) -> u64 {
    let grid = parse_input(input);
    if grid.rows() < 3 || grid.cols() < 3 {
        return 0;
    }

    let mut count = 0;
    for i_center in 1..(grid.rows() - 1) {
        for j_center in 1..(grid.cols() - 1) {
            let center = grid[i_center][j_center];
            if center != 2 {
                continue;
            }

            let ne = grid[i_center - 1][j_center + 1];
            let se = grid[i_center + 1][j_center + 1];
            let sw = grid[i_center + 1][j_center - 1];
            let nw = grid[i_center - 1][j_center - 1];

            let is_match = matches!(
                (ne, se, sw, nw),
                (1, 1, 3, 3) | (1, 3, 3, 1) | (3, 3, 1, 1) | (3, 1, 1, 3),
            );
            if is_match {
                count += 1;
            }
        }
    }
    count
}
