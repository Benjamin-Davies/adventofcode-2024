#[derive(Debug, Clone, Copy)]
enum Group {
    File { size: u8, number: u32 },
    Empty { size: u8 },
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
                    size,
                    number: i as u32 / 2,
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

fn compact(map: &mut Vec<Group>, allow_fragmentation: bool) {
    'outer: loop {
        let i = map.len() - 1;
        let Group::File { size, number } = map[i] else {
            map.pop();
            continue;
        };

        for (j, &slot) in map.iter().enumerate() {
            let Group::Empty { size: slot_size } = slot else {
                continue;
            };

            if size < slot_size {
                map[j] = Group::Empty {
                    size: slot_size - size,
                };
                map.insert(j, Group::File { size, number });
                map.pop();
                continue 'outer;
            } else if size == slot_size {
                map[j] = Group::File { size, number };
                map.pop();
                continue 'outer;
            } else if allow_fragmentation {
                map[j] = Group::File {
                    size: slot_size,
                    number,
                };
                map[i] = Group::File {
                    size: size - slot_size,
                    number,
                };
                continue 'outer;
            }
        }

        break 'outer;
    }
}

fn checksum(map: &[Group]) -> u64 {
    let mut sum = 0;
    let mut block = 0;
    for &group in map {
        match group {
            Group::File { size, number } => {
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
