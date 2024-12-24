#[derive(Debug, Clone, Copy)]
enum Group {
    File {
        number: u32,
        size: u8,
        has_moved: bool,
    },
    Empty {
        size: u8,
    },
}

fn parse_input(input: &str) -> Vec<Group> {
    input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let size = c as u8 - b'0';
            if i % 2 == 0 {
                Group::File {
                    number: i as u32 / 2,
                    size,
                    has_moved: false,
                }
            } else {
                Group::Empty { size }
            }
        })
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let mut map = parse_input(input);

    compact(&mut map, true);
    checksum(&map)
}

pub fn part2(input: &str) -> u64 {
    let mut map = parse_input(input);

    compact(&mut map, false);
    checksum(&map)
}

fn compact(map: &mut Vec<Group>, allow_fragmentation: bool) {
    'outer: for i in (0..map.len()).rev() {
        'relocate: loop {
            let Group::File {
                number,
                size,
                has_moved,
            } = map[i]
            else {
                continue 'outer;
            };
            if has_moved {
                continue 'outer;
            }

            'search: for (j, &slot) in map[..i].iter().enumerate() {
                let Group::Empty { size: slot_size } = slot else {
                    continue 'search;
                };

                if size < slot_size {
                    map[j] = Group::Empty {
                        size: slot_size - size,
                    };
                    map[i] = Group::Empty { size };
                    map.insert(
                        j,
                        Group::File {
                            number,
                            size,
                            has_moved: true,
                        },
                    );
                    continue 'outer;
                } else if size == slot_size {
                    map.swap(i, j);
                    continue 'outer;
                } else if allow_fragmentation {
                    map[j] = Group::File {
                        number,
                        size: slot_size,
                        has_moved: true,
                    };
                    map[i] = Group::File {
                        number,
                        size: size - slot_size,
                        has_moved: false,
                    };
                    continue 'relocate;
                }
            }

            break 'relocate;
        }
    }
}

fn checksum(map: &[Group]) -> u64 {
    let mut sum = 0;
    let mut block = 0;
    for &group in map {
        match group {
            Group::File { size, number, .. } => {
                let size = size as u64;
                let number = number as u64;
                if size > 0 {
                    sum += (size * block + size * (size - 1) / 2) * number;
                }
                block += size;
            }
            Group::Empty { size } => {
                block += size as u64;
            }
        }
    }
    sum
}
