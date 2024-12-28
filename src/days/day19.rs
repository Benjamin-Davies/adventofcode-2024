use regex::Regex;

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (available_part, desired_part) = input.split_once("\n\n").unwrap();

    let available = available_part.split(", ").collect();
    let desired = desired_part.lines().collect();
    (available, desired)
}

pub fn part1(input: &str) -> u64 {
    let (available, desired) = parse_input(input);

    let re = Regex::new(&format!("^({})*$", available.join("|"))).unwrap();

    desired.iter().filter(|&s| re.is_match(s)).count() as u64
}
