use std::{
    env::args,
    io::{stdin, Read},
};

use adventofcode_2024::days::get_solution;

fn main() {
    let day = args()
        .nth(1)
        .expect("day number as argument")
        .trim()
        .parse::<u32>()
        .unwrap();

    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let solution = get_solution(day);
    let answer = solution(&input);
    println!("Answer: {answer}");
}
