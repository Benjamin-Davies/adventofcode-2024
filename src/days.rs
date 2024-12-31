pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;

pub trait Solution {
    fn run(&self, input: &str) -> String;
}

impl<F, A> Solution for F
where
    F: Fn(&str) -> A,
    A: ToString,
{
    fn run(&self, input: &str) -> String {
        self(input).to_string()
    }
}

pub fn get_solution(part: &str) -> &dyn Solution {
    match part {
        "1.1" => &day01::part1,
        "1.2" => &day01::part2,
        "2.1" => &day02::part1,
        "2.2" => &day02::part2,
        "3.1" => &day03::part1,
        "3.2" => &day03::part2,
        "4.1" => &day04::part1,
        "4.2" => &day04::part2,
        "5.1" => &day05::part1,
        "5.2" => &day05::part2,
        "6.1" => &day06::part1,
        "6.2" => &day06::part2,
        "7.1" => &day07::part1,
        "7.2" => &day07::part2,
        "8.1" => &day08::part1,
        "8.2" => &day08::part2,
        "9.1" => &day09::part1,
        "9.2" => &day09::part2,
        "10.1" => &day10::part1,
        "10.2" => &day10::part2,
        "11.1" => &day11::part1,
        "11.2" => &day11::part2,
        "12.1" => &day12::part1,
        "12.2" => &day12::part2,
        "13.1" => &day13::part1,
        "13.2" => &day13::part2,
        "14.1" => &day14::part1,
        "14.2" => &day14::part2,
        "15.1" => &day15::part1,
        "15.2" => &day15::part2,
        "16.1" => &day16::part1,
        "16.2" => &day16::part2,
        "17.1" => &day17::part1,
        "17.2" => &day17::part2,
        "18.1" => &day18::part1,
        "18.2" => &day18::part2,
        "19.1" => &day19::part1,
        "19.2" => &day19::part2,
        "20.1" => &day20::part1,
        "20.2" => &day20::part2,
        "21.1" => &day21::part1,
        "21.2" => &day21::part2,
        "22.1" => &day22::part1,
        "22.2" => &day22::part2,
        "23.1" => &day23::part1,
        _ => unimplemented!("part {part}"),
    }
}
