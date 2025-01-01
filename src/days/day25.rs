type Pattern = [u8; 5];

fn parse_input(input: &str) -> (Vec<Pattern>, Vec<Pattern>) {
    let mut keys = Vec::new();
    let mut holes = Vec::new();

    for pattern_part in input.split("\n\n") {
        let mut lines = pattern_part.lines();
        let first_line = lines.next().unwrap();

        let mut pattern = [0; 5];
        for line in (&mut lines).take(5) {
            for (i, c) in line.chars().enumerate() {
                match c {
                    '#' => pattern[i] += 1,
                    '.' => {}
                    _ => unimplemented!(),
                }
            }
        }

        let last_line = lines.next().unwrap();
        assert!(lines.next().is_none());

        match (first_line, last_line) {
            (".....", "#####") => keys.push(pattern),
            ("#####", ".....") => holes.push(pattern),
            _ => unimplemented!(),
        }
    }

    (keys, holes)
}

pub fn part1(input: &str) -> u64 {
    let (keys, holes) = parse_input(input);

    keys.iter()
        .flat_map(|&key| holes.iter().filter(move |&&hole| compatible(key, hole)))
        .count() as u64
}

fn compatible(key: Pattern, hole: Pattern) -> bool {
    key.into_iter().zip(hole).all(|(x, y)| x + y <= 5)
}
