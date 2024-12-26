use crate::shared::{grid::Grid, vec2::Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Air,
    Robot,
    Box,
    Wall,
    BoxLeftHalf,
    BoxRightHalf,
}

impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        match value {
            Cell::Air => '.',
            Cell::Robot => '@',
            Cell::Box => 'O',
            Cell::Wall => '#',
            Cell::BoxLeftHalf => '[',
            Cell::BoxRightHalf => ']',
        }
    }
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

pub fn part2(input: &str) -> u64 {
    let (mut map, mut robot_pos, moves) = parse_input(input);
    map = widen(map);
    robot_pos.x *= 2;

    for &dir in &moves {
        if can_push2(robot_pos, dir, &map) {
            push2(robot_pos, dir, &mut map);
            robot_pos += dir;
        }
    }

    let mut sum = 0;
    for (y, x) in map.positions() {
        if map[y][x] == Cell::BoxLeftHalf {
            sum += 100 * y as u64 + x as u64;
        }
    }
    sum
}

/// Returns true if the push was successful.
fn try_push(pos: Vec2, dir: Vec2, map: &mut Grid<Cell>) -> bool {
    match map[pos] {
        Cell::Air => true,
        Cell::Wall => false,
        current => {
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

fn widen(map: Grid<Cell>) -> Grid<Cell> {
    let mut new_map = Grid::new(map.rows(), 2 * map.cols(), Cell::Air);
    for (i, j) in map.positions() {
        match map[i][j] {
            Cell::Robot => {
                new_map[i][2 * j] = Cell::Robot;
            }
            Cell::Box => {
                new_map[i][2 * j] = Cell::BoxLeftHalf;
                new_map[i][2 * j + 1] = Cell::BoxRightHalf;
            }
            cell => {
                new_map[i][2 * j] = cell;
                new_map[i][2 * j + 1] = cell;
            }
        }
    }
    new_map
}

fn can_push2(pos: Vec2, dir: Vec2, map: &Grid<Cell>) -> bool {
    match map[pos] {
        Cell::Air => true,
        Cell::Wall => false,
        Cell::BoxLeftHalf => {
            let other_half = pos + Vec2 { x: 1, y: 0 };
            let next_pos = pos + dir;
            can_push2(next_pos, dir, map) && (dir.y == 0 || can_push2(other_half + dir, dir, map))
        }
        Cell::BoxRightHalf => {
            let other_half = pos + Vec2 { x: -1, y: 0 };
            let next_pos = pos + dir;
            can_push2(next_pos, dir, map) && (dir.y == 0 || can_push2(other_half + dir, dir, map))
        }
        _ => {
            let next_pos = pos + dir;
            can_push2(next_pos, dir, map)
        }
    }
}

fn push2(pos: Vec2, dir: Vec2, map: &mut Grid<Cell>) {
    match map[pos] {
        Cell::Air => {}
        Cell::Wall => unreachable!(),
        Cell::BoxLeftHalf => push2_double(pos, dir, map),
        Cell::BoxRightHalf => push2_double(pos + Vec2 { x: -1, y: 0 }, dir, map),
        current => {
            let next_pos = pos + dir;
            push2(next_pos, dir, map);
            map[next_pos] = current;
            map[pos] = Cell::Air;
        }
    }
}

fn push2_double(pos: Vec2, dir: Vec2, map: &mut Grid<Cell>) {
    let current_left = pos;
    let current_right = current_left + Vec2 { x: 1, y: 0 };

    let next_left = current_left + dir;
    let next_right = current_right + dir;

    match dir {
        Vec2 { x: -1, y: 0 } => push2(next_left, dir, map),
        Vec2 { x: 1, y: 0 } => push2(next_right, dir, map),
        _ => {
            push2(next_left, dir, map);
            push2(next_right, dir, map);
        }
    }

    map[current_left] = Cell::Air;
    map[current_right] = Cell::Air;
    map[next_left] = Cell::BoxLeftHalf;
    map[next_right] = Cell::BoxRightHalf;
}
