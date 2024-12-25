use crate::shared::{grid::Grid, vec2::Vec2};

fn parse_input(input: &str) -> Vec<(Vec2, Vec2)> {
    input
        .lines()
        .map(|line| {
            let (p_part, v_part) = line.split_once(' ').unwrap();
            let p = parse_vec2(p_part.strip_prefix("p=").unwrap());
            let v = parse_vec2(v_part.strip_prefix("v=").unwrap());
            (p, v)
        })
        .collect()
}

fn parse_vec2(input: &str) -> Vec2 {
    let (x_part, y_part) = input.split_once(',').unwrap();
    let x = x_part.parse().unwrap();
    let y = y_part.parse().unwrap();
    Vec2 { x, y }
}

pub fn part1(input: &str) -> u64 {
    let mut robots = parse_input(input);

    let size = guess_size(&robots);
    step_n(&mut robots, size, 100);

    safety_factor(&robots, size)
}

pub fn part2(input: &str) -> u64 {
    let mut robots = parse_input(input);

    let size = guess_size(&robots);

    let mut steps = 0;
    while has_overlap(&robots, size) {
        step_n(&mut robots, size, 1);
        steps += 1;
    }

    display_robots(&robots, size);

    steps
}

fn guess_size(robots: &[(Vec2, Vec2)]) -> Vec2 {
    Vec2 {
        x: robots.iter().map(|(p, _)| p.x + 1).max().unwrap(),
        y: robots.iter().map(|(p, _)| p.y + 1).max().unwrap(),
    }
}

fn step_n(robots: &mut [(Vec2, Vec2)], size: Vec2, n: i64) {
    for (p, v) in robots {
        *p = (*p + n * *v) % size;
    }
}

fn safety_factor(robots: &[(Vec2, Vec2)], size: Vec2) -> u64 {
    let half_w = size.x / 2;
    let half_h = size.y / 2;

    let nw_count = robots
        .iter()
        .filter(|(p, _)| p.x < half_w && p.y < half_h)
        .count() as u64;
    let ne_count = robots
        .iter()
        .filter(|(p, _)| p.x > half_w && p.y < half_h)
        .count() as u64;
    let sw_count = robots
        .iter()
        .filter(|(p, _)| p.x < half_w && p.y > half_h)
        .count() as u64;
    let se_count = robots
        .iter()
        .filter(|(p, _)| p.x > half_w && p.y > half_h)
        .count() as u64;

    nw_count * ne_count * sw_count * se_count
}

fn has_overlap(robots: &[(Vec2, Vec2)], size: Vec2) -> bool {
    let mut grid = Grid::new(size.y as usize, size.x as usize, false);
    for (p, _) in robots {
        let visited = &mut grid[p.y as usize][p.x as usize];
        if *visited {
            return true;
        } else {
            *visited = true;
        }
    }
    false
}

fn display_robots(robots: &[(Vec2, Vec2)], size: Vec2) {
    let mut grid = Grid::new(size.y as usize, size.x as usize, 0);
    for (p, _) in robots {
        grid[p.y as usize][p.x as usize] += 1;
    }

    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            let n = grid[i][j];
            if n == 0 {
                print!(".");
            } else {
                print!("{n}");
            }
        }
        println!();
    }
}
