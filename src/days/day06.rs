use crate::shared::grid::Grid;

#[derive(Debug, Default, Clone)]
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

    fn next(&mut self, obstacles: &Grid<bool>) {
        let (i_next, j_next) = self.next_position();
        if obstacles.contains(i_next, j_next) && obstacles[i_next as usize][j_next as usize] {
            self.turn_right();
        } else {
            self.step_forward();
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

    fn direction_ordinal(&self) -> u8 {
        match (self.i_dir, self.j_dir) {
            (1, 0) => 0,
            (0, 1) => 1,
            (-1, 0) => 2,
            (0, -1) => 3,
            _ => unimplemented!(),
        }
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
        guard.next(&obstacles);
    }

    visited.positions().filter(|&(i, j)| visited[i][j]).count() as u64
}

pub fn part2(input: &str) -> u64 {
    let (obstacles, mut guard) = parse_input(input);

    let mut visited = Grid::new(obstacles.rows(), obstacles.cols(), 0);
    let mut visited2 = Grid::new(obstacles.rows(), obstacles.cols(), 0);
    let mut obstacles2 = obstacles.clone();
    let mut possible_obstacle_count = 0;
    while obstacles.contains(guard.i, guard.j) {
        visited[guard.i as usize][guard.j as usize] |= 1 << guard.direction_ordinal();

        let (i_next, j_next) = guard.next_position();
        if !obstacles.contains(i_next, j_next) {
            break;
        }
        let i_next = i_next as usize;
        let j_next = j_next as usize;
        if obstacles[i_next][j_next] == false && visited[i_next][j_next] == 0 {
            let mut guard2 = guard.clone();
            visited2.copy_from(&visited);
            obstacles2[i_next][j_next] = true;
            guard2.next(&obstacles2);

            if detect_cycle(&mut guard2, &mut visited2, &obstacles2) {
                possible_obstacle_count += 1;
            }

            obstacles2[i_next][j_next] = false;
        }

        guard.next(&obstacles);
    }

    possible_obstacle_count
}

fn detect_cycle(guard: &mut Guard, visited: &mut Grid<u8>, obstacles: &Grid<bool>) -> bool {
    while obstacles.contains(guard.i, guard.j) {
        let i = guard.i as usize;
        let j = guard.j as usize;
        let dir_bitmask = 1 << guard.direction_ordinal();
        if (visited[i][j] & dir_bitmask) != 0 {
            return true;
        }
        visited[i][j] |= dir_bitmask;

        guard.next(obstacles);
    }

    false
}
