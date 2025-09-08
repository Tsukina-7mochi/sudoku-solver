use std::{fmt::Display, ops::Index};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sudoku<const N: usize> {
    pub cells: [[u8; N]; N],
}

impl<const N: usize> Sudoku<N> {
    pub fn new(cells: [[u8; N]; N]) -> Self {
        Self {
            cells: cells.map(|row| row.map(|v| v - 1)),
        }
    }
}

impl<const N: usize> Display for Sudoku<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.cells.iter().enumerate() {
            for cell in row {
                write!(f, "{cell}")?;
            }
            if y != N - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl<const N: usize> Index<(usize, usize)> for Sudoku<N> {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (y, x) = index;
        &self.cells[y][x]
    }
}

impl<const N: usize> From<[[u8; N]; N]> for Sudoku<N> {
    fn from(cells: [[u8; N]; N]) -> Self {
        Self { cells }
    }
}
