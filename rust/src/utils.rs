use std::{
    fmt::{Display, Write},
    str::Lines,
};

use itertools::Itertools;

pub struct Grid2D<T> {
    height: usize,
    width: usize,
    rows: Vec<Vec<T>>,
}

impl Grid2D<char> {
    pub fn from_lines(lines: Lines) -> Self {
        let vec = lines
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self {
            height: vec.len(),
            width: vec.first().unwrap_or(&Vec::new()).len(),
            rows: vec,
        }
    }

    pub fn to_string(&self) -> String {
        self.rows
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n")
    }
}

impl<T> Display for Grid2D<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            for item in row.iter() {
                f.write_fmt(format_args!("{item}"))?;
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl<T> Grid2D<T> {
    pub fn new(rows: Vec<Vec<T>>) -> Self {
        Self {
            height: rows.len(),
            width: rows.first().unwrap_or(&vec![]).len(),
            rows,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x > self.width || y > self.height {
            return None;
        }

        Some(self.rows.get(y)?.get(x)?)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        Some(self.rows.get_mut(y)?.get_mut(x)?)
    }

    pub fn replace(&mut self, item: T, x: usize, y: usize) {
        if x > self.width || y > self.height {
            return;
        }

        let row = self.rows.get_mut(y).unwrap();
        let x = row.get_mut(x).unwrap();
        *x = item;
    }
}

impl<T> Grid2D<T>
where
    T: Clone,
{
    pub fn with_size(width: usize, height: usize, default: T) -> Self {
        Self {
            height,
            width,
            rows: vec![vec![default; width]; height],
        }
    }
}

impl<'b, T> Grid2D<T> {
    pub fn row_iterator<'a>(&self, row_index: usize) -> RowIterator<T> {
        RowIterator {
            grid: self,
            row_index,
            index: 0,
        }
    }

    pub fn col_iterator<'a>(&self, column_index: usize) -> ColumnIterator<T> {
        ColumnIterator {
            grid: self,
            column_index,
            index: 0,
        }
    }
}

impl<T> Clone for Grid2D<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            rows: self.rows.clone(),
        }
    }
}

pub struct ColumnIterator<'a, T> {
    grid: &'a Grid2D<T>,
    column_index: usize,
    index: usize,
}

impl<'a, T> Iterator for ColumnIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.grid.height {
            return None;
        }

        self.index += 1;

        let row = self.grid.rows.get(self.index - 1)?;
        Some(row.get(self.column_index)?)
    }
}

pub struct RowIterator<'a, T> {
    grid: &'a Grid2D<T>,
    row_index: usize,
    index: usize,
}

impl<'a, T> Iterator for RowIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.grid.width {
            return None;
        }

        self.index += 1;

        let row = self.grid.rows.get(self.row_index)?;
        Some(row.get(self.index - 1)?)
    }
}
