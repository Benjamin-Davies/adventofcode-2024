use adventofcode_2024::days::day01;

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
