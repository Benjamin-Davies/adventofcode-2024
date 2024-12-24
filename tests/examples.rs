macro_rules! tests {
    (
        $( $day:ident { $( $part:ident { $(
            $example:ident : $answer:literal
        ),* }, )* }, )*
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
                assert_eq!(answer, $answer);
            }
        )* } )* } )*
    };
}

tests! {
    day01 {
        part1 { ex1: 11 },
        part2 { ex1: 31 },
    },
    day02 {
        part1 { ex1: 2 },
        part2 { ex1: 4 },
    },
    day03 {
        part1 { ex1: 161 },
        part2 { ex2: 48 },
    },
    day04 {
        part1 { ex1: 4, ex2: 18 },
        part2 { ex2: 9 },
    },
    day05 {
        part1 { ex1: 143 },
        part2 { ex1: 123 },
    },
    day06 {
        part1 { ex1: 41 },
        part2 { ex1: 6 },
    },
    day07 {
        part1 { ex1: 3749 },
        part2 { ex1: 11387 },
    },
    day08 {
        part1 { ex1: 14 },
        part2 { ex1: 34 },
    },
    day09 {
        part1 { ex1: 1928, ex2: 60 },
        part2 { ex1: 2858 },
    },
    day10 {
        part1 { ex1: 1, ex2: 2, ex3: 4, ex4: 3 },
    },
}
