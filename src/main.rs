use std::{
    env::args,
    io::{stdin, Read},
};

use adventofcode_2024::days::get_solution;

fn main() {
    let part = args().nth(1).expect("day number as argument");
    let solution = get_solution(&part);

    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let answer = solution.run(&input);
    println!("Answer: {answer}");
}
