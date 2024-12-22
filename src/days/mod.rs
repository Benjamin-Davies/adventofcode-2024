pub mod day01;

pub type Solution = fn(input: &str) -> u64;

pub fn get_solution(part: &str) -> Solution {
    match part {
        "1.1" => day01::part1,
        "1.2" => day01::part2,
        _ => unimplemented!("part {part}"),
    }
}
