macro_rules! tests {
    (
        $( $day:ident ( $( $part:ident { $(
            $example:ident : $answer:literal
        ),* }, )* ), )*
    ) => {
        $( mod $day { $( mod $part { $(
            #[test]
            fn $example() {
                let input = include_str!(
                    concat!(
                        "../inputs/",
                        stringify!($day),
                        "/",
                        stringify!($example),
                        ".txt",
                    ),
                );
                let answer = adventofcode_2024::days::$day::$part(input);
                assert_eq!(answer.to_string(), $answer.to_string());
            }
        )* } )* } )*
    };
}

tests! {
    day01(
        part1 { ex1: 11 },
        part2 { ex1: 31 },
    ),
    day02(
        part1 { ex1: 2 },
        part2 { ex1: 4 },
    ),
    day03(
        part1 { ex1: 161 },
        part2 { ex2: 48 },
    ),
    day04(
        part1 { ex1: 4, ex2: 18 },
        part2 { ex2: 9 },
    ),
    day05(
        part1 { ex1: 143 },
        part2 { ex1: 123 },
    ),
    day06(
        part1 { ex1: 41 },
        part2 { ex1: 6 },
    ),
    day07(
        part1 { ex1: 3749 },
        part2 { ex1: 11387 },
    ),
    day08(
        part1 { ex1: 14 },
        part2 { ex1: 34 },
    ),
    day09(
        part1 { ex1: 1928, ex2: 60 },
        part2 { ex1: 2858 },
    ),
    day10(
        part1 { ex1: 1, ex2: 2, ex3: 4, ex4: 3 },
        part2 { ex5: 81, ex6: 3, ex7: 13, ex8: 227 },
    ),
    day11(
        part1 { ex1: 55312 },
        part2 { ex1: 65601038650482u64 },
    ),
    day12(
        part1 { ex1: 140, ex2: 772, ex3: 1930 },
        part2 { ex1: 80, ex2: 436, ex3: 1206, ex4: 236, ex5: 368 },
    ),
    day13(
        part1 { ex1: 480 },
        part2 { ex1: 875318608908u64 },
    ),
    day14(
        part1 { ex1: 12 },
        part2 { ex1: 1 },
    ),
    day15(
        part1 { ex1: 10092, ex2: 2028 },
        part2 { ex1: 9021, ex3: 618 },
    ),
    day16(
        part1 { ex1: 7036, ex2: 11048 },
        part2 { ex1: 45, ex2: 64 },
    ),
    day17(
        part1 { ex1: "4,6,3,5,6,3,5,2,1,0" },
        part2 { ex2: 117440 },
    ),
    day18(
        part1 { ex1: 22 },
        part2 { ex1: "6,1" },
    ),
    day19(
        part1 { ex1: 6 },
        part2 { ex1: 16 },
    ),
    day20(
        part1 { ex1: 5 },
        part2 { ex1: 285 },
    ),
    day21(
        part1 { ex1: 126384 },
        part2 { ex1: 154115708116294u64 },
    ),
    day22(
        part1 { ex1: 37327623 },
        part2 { ex2: 23 },
    ),
}
