use std::str::Lines;

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
}

impl<T> Grid2D<T> {
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

        for y in self.rows.iter().skip(y) {
            for x in y.iter().skip(x) {
                return Some(x);
            }
        }

        None
    }

    pub fn replace(&mut self, item: T, x: usize, y: usize) {
        if x > self.width || y > self.height {
            return;
        }

        for y in self.rows.iter_mut().skip(y) {
            for x in y.iter_mut().skip(x) {
                *x = item;
                return;
            }
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

pub struct ColumnIterator<'a, T> {
    grid: &'a Grid2D<T>,
    column_index: usize,
    index: usize,
}

impl<'a, T> Iterator for ColumnIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.grid.width {
            return None;
        }

        self.index += 1;

        for item in self.grid.rows.iter().skip(self.index - 1) {
            for col in item.iter().skip(self.column_index) {
                return Some(col);
            }
        }

        return None;
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

        for item in self.grid.rows.iter().skip(self.row_index) {
            for col in item.iter().skip(self.index - 1) {
                return Some(col);
            }
        }

        return None;
    }
}
