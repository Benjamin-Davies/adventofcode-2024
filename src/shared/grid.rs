use std::ops::{Index, IndexMut};

pub struct Grid<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

/// Allows a grid to be constructed by calling `push` and `newline` in the same
/// order that the characters appear in the puzzle input.
pub struct GridBuilder<T> {
    data: Vec<T>,
    cols: Option<usize>,
}

impl<T> Grid<T> {
    pub fn new(rows: usize, cols: usize, value: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![value; rows * cols],
            rows,
            cols,
        }
    }

    pub fn builder() -> GridBuilder<T> {
        GridBuilder {
            data: Vec::new(),
            cols: None,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
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

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.cols;
        &mut self.data[start..start + self.cols]
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

    pub fn next_position(&self) -> (usize, usize) {
        if let Some(cols) = self.cols {
            (self.data.len() / cols, self.data.len() % cols)
        } else {
            (0, self.data.len())
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
