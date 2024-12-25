use crate::shared::{grid::Grid, vec2::Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Air,
    Robot,
    Box,
    Wall,
}

fn parse_input(input: &str) -> (Grid<Cell>, Vec2, Vec<Vec2>) {
    let (map_part, moves_part) = input.split_once("\n\n").unwrap();

    let mut map = Grid::builder();
    let mut robot_pos = None;
    for c in map_part.chars() {
        match c {
            '\n' => map.newline(),
            '.' => map.push(Cell::Air),
            '@' => {
                let (y, x) = map.next_position();
                robot_pos = Some(Vec2 {
                    x: x as i64,
                    y: y as i64,
                });
                map.push(Cell::Robot);
            }
            'O' => map.push(Cell::Box),
            '#' => map.push(Cell::Wall),
            _ => unimplemented!(),
        }
    }
    map.newline();
    let map = map.build();
    let robot_pos = robot_pos.unwrap();

    let moves_part = moves_part
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '^' => Vec2 { x: 0, y: -1 },
            '>' => Vec2 { x: 1, y: 0 },
            'v' => Vec2 { x: 0, y: 1 },
            '<' => Vec2 { x: -1, y: 0 },
            _ => unimplemented!(),
        })
        .collect();

    (map, robot_pos, moves_part)
}

pub fn part1(input: &str) -> u64 {
    let (mut map, mut robot_pos, moves) = parse_input(input);

    for &dir in &moves {
        if try_push(robot_pos, dir, &mut map) {
            robot_pos += dir;
        }
    }

    let mut sum = 0;
    for (y, x) in map.positions() {
        if map[y][x] == Cell::Box {
            sum += 100 * y as u64 + x as u64;
        }
    }
    sum
}

/// Returns true if the push was successful.
fn try_push(pos: Vec2, dir: Vec2, map: &mut Grid<Cell>) -> bool {
    let current = map[pos];
    match current {
        Cell::Air => true,
        Cell::Wall => false,
        _ => {
            let next_pos = pos + dir;
            if try_push(next_pos, dir, map) {
                map[next_pos] = current;
                map[pos] = Cell::Air;
                true
            } else {
                false
            }
        }
    }
}
