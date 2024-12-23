use adventofcode_2024::days::{day01, day02, day03, day04, day05, day06};

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

#[test]
fn day03_1_ex1() {
    let input = include_str!("../inputs/day03/ex1.txt");
    let answer = day03::part1(input);
    assert_eq!(answer, 161);
}

#[test]
fn day03_2_ex2() {
    let input = include_str!("../inputs/day03/ex2.txt");
    let answer = day03::part2(input);
    assert_eq!(answer, 48);
}

#[test]
fn day04_1_ex1() {
    let input = include_str!("../inputs/day04/ex1.txt");
    let answer = day04::part1(input);
    assert_eq!(answer, 4);
}

#[test]
fn day04_1_ex2() {
    let input = include_str!("../inputs/day04/ex2.txt");
    let answer = day04::part1(input);
    assert_eq!(answer, 18);
}

#[test]
fn day04_2_ex2() {
    let input = include_str!("../inputs/day04/ex2.txt");
    let answer = day04::part2(input);
    assert_eq!(answer, 9);
}

#[test]
fn day05_1_ex1() {
    let input = include_str!("../inputs/day05/ex1.txt");
    let answer = day05::part1(input);
    assert_eq!(answer, 143);
}

#[test]
fn day05_2_ex1() {
    let input = include_str!("../inputs/day05/ex1.txt");
    let answer = day05::part2(input);
    assert_eq!(answer, 123);
}

#[test]
fn day06_1_ex1() {
    let input = include_str!("../inputs/day06/ex1.txt");
    let answer = day06::part1(input);
    assert_eq!(answer, 41);
}

#[test]
fn day06_2_ex1() {
    let input = include_str!("../inputs/day06/ex1.txt");
    let answer = day06::part2(input);
    assert_eq!(answer, 6);
}
