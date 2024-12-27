use std::{
    fmt::{self, Write},
    ops::{Index, IndexMut},
};

use super::vec2::Vec2;

#[derive(Clone)]
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
        cartesian_product(0..self.rows, 0..self.cols)
    }

    pub fn contains(&self, i: isize, j: isize) -> bool {
        i >= 0 && j >= 0 && i < self.rows as isize && j < self.cols as isize
    }

    pub fn copy_from(&mut self, other: &Grid<T>)
    where
        T: Copy,
    {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        self.data.copy_from_slice(&other.data);
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

impl<T> Index<Vec2> for Grid<T> {
    type Output = T;

    fn index(&self, index: Vec2) -> &Self::Output {
        assert!(index.y >= 0);
        assert!(index.x >= 0);
        &self[index.y as usize][index.x as usize]
    }
}

impl<T> IndexMut<Vec2> for Grid<T> {
    fn index_mut(&mut self, index: Vec2) -> &mut Self::Output {
        assert!(index.y >= 0);
        assert!(index.x >= 0);
        &mut self[index.y as usize][index.x as usize]
    }
}

impl<T> fmt::Debug for Grid<T>
where
    T: Copy + Into<char>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.rows, self.cols)?;
        for i in 0..self.rows {
            writeln!(f)?;
            for j in 0..self.cols {
                f.write_char(self[i][j].into())?;
            }
        }
        Ok(())
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

    pub fn next_position_vec2(&self) -> Vec2 {
        let (i, j) = self.next_position();
        Vec2 {
            x: j as i64,
            y: i as i64,
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

pub fn cartesian_product<'a, X: Clone, Y>(
    a: impl Iterator<Item = X> + 'a,
    b: impl Iterator<Item = Y> + Clone + 'a,
) -> impl Iterator<Item = (X, Y)> {
    a.flat_map(move |x| b.clone().map(move |y| (x.clone(), y)))
}
