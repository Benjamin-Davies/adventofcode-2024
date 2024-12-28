use crate::shared::{
    grid::{cartesian_product, Grid},
    vec2::Vec2,
};

fn parse_input(input: &str) -> (Grid<bool>, Vec2, Vec2) {
    let mut map = Grid::builder();
    let mut start = None;
    let mut end = None;
    for c in input.chars() {
        match c {
            '\n' => map.newline(),
            '.' => map.push(false),
            'S' => {
                start = Some(map.next_position_vec2());
                map.push(false);
            }
            'E' => {
                end = Some(map.next_position_vec2());
                map.push(false);
            }
            '#' => map.push(true),
            _ => unimplemented!(),
        }
    }
    (map.build(), start.unwrap(), end.unwrap())
}

pub fn part1(input: &str) -> u64 {
    let (map, start, _end) = parse_input(input);

    let min = if map.rows() <= 20 { 20 } else { 100 };

    let distances = find_distances(&map, start);

    shortcuts(&distances)
        .filter(|&(_, _, saves)| saves >= min)
        .count() as u64
}

pub fn part2(input: &str) -> u64 {
    let (map, start, _end) = parse_input(input);

    let min = if map.rows() <= 20 { 50 } else { 100 };

    let distances = find_distances(&map, start);

    shortcuts2(&distances)
        .filter(|&(_, _, saves)| saves >= min)
        .count() as u64
}

fn find_distances(map: &Grid<bool>, start: Vec2) -> Grid<u64> {
    let mut distances = Grid::new(map.rows(), map.cols(), u64::MAX);
    let mut pos = start;
    let mut distance = 0;
    'outer: loop {
        distances[pos] = distance;

        distance += 1;
        for dir in Vec2::CARDINAL_DIRECTIONS {
            let new_pos = pos + dir;
            if map.contains(new_pos.y as isize, new_pos.x as isize)
                && !map[new_pos]
                && distances[new_pos] == u64::MAX
            {
                pos = new_pos;
                continue 'outer;
            }
        }
        break 'outer;
    }
    distances
}

fn shortcuts(distances: &Grid<u64>) -> impl Iterator<Item = (Vec2, Vec2, u64)> + '_ {
    distances.positions().flat_map(move |(i, j)| {
        let p1 = Vec2 {
            x: j as i64,
            y: i as i64,
        };

        Vec2::CARDINAL_DIRECTIONS
            .into_iter()
            .filter_map(move |dir| {
                let p2 = p1 + 2 * dir;

                if !distances.contains(p2.y as isize, p2.x as isize) {
                    return None;
                }
                if distances[p1] == u64::MAX || distances[p2] == u64::MAX {
                    return None;
                }
                if distances[p1] + 2 >= distances[p2] {
                    return None;
                }

                Some((p1, p2, distances[p2] - distances[p1] - 2))
            })
    })
}

fn shortcuts2(distances: &Grid<u64>) -> impl Iterator<Item = (Vec2, Vec2, u64)> + '_ {
    distances.positions().flat_map(move |(i, j)| {
        let p1 = Vec2 {
            x: j as i64,
            y: i as i64,
        };

        cartesian_product(-20..=20i64, -20..=20i64).filter_map(move |(dx, dy)| {
            if dx.abs() + dy.abs() > 20 {
                return None;
            }
            let delta = Vec2 { x: dx, y: dy };

            let p2 = p1 + delta;
            let new_time = dx.unsigned_abs() + dy.unsigned_abs();

            if !distances.contains(p2.y as isize, p2.x as isize) {
                return None;
            }
            if distances[p1] == u64::MAX || distances[p2] == u64::MAX {
                return None;
            }
            if distances[p1] + new_time >= distances[p2] {
                return None;
            }

            Some((p1, p2, distances[p2] - distances[p1] - new_time))
        })
    })
}
