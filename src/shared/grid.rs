use std::ops::Index;

pub struct Grid<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

pub struct GridBuilder<T> {
    data: Vec<T>,
    cols: Option<usize>,
}

impl<T> Grid<T> {
    pub fn builder() -> GridBuilder<T> {
        GridBuilder {
            data: Vec::new(),
            cols: None,
        }
    }

    pub fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
        let rows = self.rows;
        let cols = self.cols;
        (0..rows).flat_map(move |i| (0..cols).map(move |j| (i, j)))
    }

    pub fn contains(&self, i: isize, j: isize) -> bool {
        i >= 0 && j >= 0 && i < self.rows as isize && j < self.cols as isize
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.cols;
        &self.data[start..start + self.cols]
    }
}

impl<T> GridBuilder<T> {
    pub fn push(&mut self, x: T) {
        self.data.push(x);
    }

    pub fn newline(&mut self) {
        if let Some(width) = self.cols {
            assert_eq!(self.data.len() % width, 0);
        } else {
            self.cols = Some(self.data.len());
        }
    }

    pub fn build(self) -> Grid<T> {
        let width = self.cols.unwrap();
        let height = self.data.len() / width;
        Grid {
            data: self.data,
            cols: width,
            rows: height,
        }
    }
}
