use crate::shared::vec2::Vec2;

#[derive(Debug)]
struct ClawMachine {
    button_a: Vec2,
    button_b: Vec2,
    prize: Vec2,
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut lines = input.lines();
    let mut machines = Vec::new();
    loop {
        let button_a_part = lines.next().unwrap().strip_prefix("Button A: ").unwrap();
        let button_a = parse_vec2(button_a_part);

        let button_b_part = lines.next().unwrap().strip_prefix("Button B: ").unwrap();
        let button_b = parse_vec2(button_b_part);

        let prize_part = lines.next().unwrap().strip_prefix("Prize: ").unwrap();
        let prize = parse_vec2(prize_part);

        machines.push(ClawMachine {
            button_a,
            button_b,
            prize,
        });

        if lines.next().is_none() {
            break;
        }
    }

    machines
}

fn parse_vec2(input: &str) -> Vec2 {
    let (x_part, y_part) = input.split_once(", ").unwrap();
    let x = x_part[2..].parse().unwrap();
    let y = y_part[2..].parse().unwrap();
    Vec2 { x, y }
}

pub fn part1(input: &str) -> u64 {
    let machines = parse_input(input);

    machines.iter().filter_map(tokens_to_win).sum()
}

pub fn part2(input: &str) -> u64 {
    let mut machines = parse_input(input);

    for machine in &mut machines {
        machine.prize.x += 10_000_000_000_000;
        machine.prize.y += 10_000_000_000_000;
    }

    machines.iter().filter_map(tokens_to_win).sum()
}

fn tokens_to_win(machine: &ClawMachine) -> Option<u64> {
    let &ClawMachine {
        button_a,
        button_b,
        prize,
    } = machine;

    // Find an integer solution to:
    // a*x_a + b*x_b = x_p
    // a*y_a + b*y_b = y_p

    // The following solves using a 2x2 matrix inversion but defers the division until last.
    let a_numerator = prize.x * button_b.y - prize.y * button_b.x;
    let b_numerator = -prize.x * button_a.y + prize.y * button_a.x;
    let determinant = button_a.x * button_b.y - button_b.x * button_a.y;

    // This is true in all of the example cases, and is necessary for there to be a unique solution.
    assert_ne!(determinant, 0);

    if a_numerator % determinant != 0 || b_numerator % determinant != 0 {
        return None;
    }
    let a = (a_numerator / determinant) as u64;
    let b = (b_numerator / determinant) as u64;

    // Return the number of tokens spent.
    Some(3 * a + b)
}
