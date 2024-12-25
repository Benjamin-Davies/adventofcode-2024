use crate::shared::vec2::Vec2;

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

    let size = Vec2 {
        x: robots.iter().map(|(p, _)| p.x + 1).max().unwrap(),
        y: robots.iter().map(|(p, _)| p.y + 1).max().unwrap(),
    };

    for (p, v) in &mut robots {
        *p = (*p + 100 * *v) % size;
    }

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
