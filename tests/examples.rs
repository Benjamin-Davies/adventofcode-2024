use adventofcode_2024::days::day01;

#[test]
fn day01_ex1() {
    let input = include_str!("../inputs/day01/ex1.txt");
    let answer = day01::solution(input);
    assert_eq!(answer, 11);
}
