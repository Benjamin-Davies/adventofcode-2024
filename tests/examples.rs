use adventofcode_2024::days::{day01, day02};

#[test]
fn day01_1_ex1() {
    let input = include_str!("../inputs/day01/ex1.txt");
    let answer = day01::part1(input);
    assert_eq!(answer, 11);
}

#[test]
fn day01_2_ex1() {
    let input = include_str!("../inputs/day01/ex1.txt");
    let answer = day01::part2(input);
    assert_eq!(answer, 31);
}

#[test]
fn day02_1_ex1() {
    let input = include_str!("../inputs/day02/ex1.txt");
    let answer = day02::part1(input);
    assert_eq!(answer, 2);
}

#[test]
fn day02_2_ex1() {
    let input = include_str!("../inputs/day02/ex1.txt");
    let answer = day02::part2(input);
    assert_eq!(answer, 4);
}
