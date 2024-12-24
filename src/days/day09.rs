use std::{cmp::Ordering, fmt, iter};

fn parse_input(input: &str) -> Vec<u8> {
    let mut bytes = input.trim().as_bytes().to_vec();
    assert_eq!(bytes.len() % 2, 1);
    for b in &mut bytes {
        assert!(b.is_ascii_digit());
        *b -= b'0';
    }
    bytes
}

struct Cursor<'a> {
    map: &'a [u8],
    group: usize,
    block: usize,
}

impl<'a> Cursor<'a> {
    fn start_of(map: &'a [u8]) -> Self {
        Self {
            map,
            group: 0,
            block: 0,
        }
    }

    fn end_of(map: &'a [u8]) -> Self {
        let group = map.len() - 1;
        let block = map[group] as usize - 1;
        Self { map, group, block }
    }

    fn is_empty(&self) -> bool {
        self.group % 2 == 1
    }

    fn file_number(&self) -> u64 {
        self.group as u64 / 2
    }

    fn next_block(&mut self) {
        self.block += 1;
        while self.block >= self.map[self.group] as usize {
            self.group += 1;
            self.block = 0;
        }
    }

    fn prev_block(&mut self) {
        while self.block == 0 {
            self.group -= 1;
            self.block = self.map[self.group] as usize;
        }
        self.block -= 1;
    }
}

impl<'a> PartialEq for Cursor<'a> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&(self.group, self.block), &(other.group, other.block))
    }
}

impl<'a> PartialOrd for Cursor<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&(self.group, self.block), &(other.group, other.block))
    }
}

impl<'a> fmt::Debug for Cursor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.group, self.block)
    }
}

pub fn part1(input: &str) -> u64 {
    let map = parse_input(input);

    compacted(&map).enumerate().map(|(i, n)| i as u64 * n).sum()
}

fn compacted(map: &[u8]) -> impl Iterator<Item = u64> + '_ {
    let mut left_cursor = Cursor::start_of(&map);
    let mut right_cursor = Cursor::end_of(&map);
    iter::from_fn(move || {
        if left_cursor > right_cursor {
            return None;
        }
        let file_number;
        if !left_cursor.is_empty() {
            file_number = left_cursor.file_number();
            left_cursor.next_block();
        } else {
            while right_cursor.is_empty() {
                right_cursor.prev_block();
            }
            if left_cursor > right_cursor {
                return None;
            }

            file_number = right_cursor.file_number();
            left_cursor.next_block();
            right_cursor.prev_block();
        }

        return Some(file_number);
    })
}
