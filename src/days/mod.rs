pub mod day01;

pub type Solution = fn(input: &str) -> u64;

pub fn get_solution(day: u32) -> Solution {
    match day {
        1 => day01::solution,
        _ => unimplemented!("day {day}"),
    }
}
