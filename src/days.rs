pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

pub type Solution = fn(input: &str) -> u64;

pub fn get_solution(part: &str) -> Solution {
    match part {
        "1.1" => day01::part1,
        "1.2" => day01::part2,
        "2.1" => day02::part1,
        "2.2" => day02::part2,
        "3.1" => day03::part1,
        "3.2" => day03::part2,
        "4.1" => day04::part1,
        "4.2" => day04::part2,
        "5.1" => day05::part1,
        "5.2" => day05::part2,
        "6.1" => day06::part1,
        _ => unimplemented!("part {part}"),
    }
}
