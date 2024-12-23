use crate::shared::grid::Grid;

#[derive(Debug, Default)]
struct Guard {
    i: isize,
    j: isize,
    i_dir: isize,
    j_dir: isize,
}

impl Guard {
    fn new((i, j): (usize, usize)) -> Self {
        Self {
            i: i as isize,
            j: j as isize,
            i_dir: -1,
            j_dir: 0,
        }
    }

    fn next_position(&self) -> (isize, isize) {
        (self.i + self.i_dir, self.j + self.j_dir)
    }

    fn step_forward(&mut self) {
        self.i += self.i_dir;
        self.j += self.j_dir;
    }

    fn turn_right(&mut self) {
        (self.i_dir, self.j_dir) = (self.j_dir, -self.i_dir);
    }
}

fn parse_input(input: &str) -> (Grid<bool>, Guard) {
    let mut obstacles = Grid::builder();
    let mut guard = Guard::default();
    for c in input.chars() {
        match c {
            '\n' => obstacles.newline(),
            '#' => obstacles.push(true),
            '.' => obstacles.push(false),
            '^' => {
                guard = Guard::new(obstacles.next_position());
                obstacles.push(false);
            }
            _ => unimplemented!("{c:?}"),
        }
    }

    (obstacles.build(), guard)
}

pub fn part1(input: &str) -> u64 {
    let (obstacles, mut guard) = parse_input(input);

    let mut visited = Grid::new(obstacles.rows(), obstacles.cols(), false);
    while obstacles.contains(guard.i, guard.j) {
        visited[guard.i as usize][guard.j as usize] = true;

        let (i_next, j_next) = guard.next_position();
        if obstacles.contains(i_next, j_next) && obstacles[i_next as usize][j_next as usize] {
            guard.turn_right();
        } else {
            guard.step_forward();
        }
    }

    visited.positions().filter(|&(i, j)| visited[i][j]).count() as u64
}
