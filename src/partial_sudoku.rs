use std::{fmt::Display, ops::Index};

use crate::sudoku::Sudoku;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartialSudoku<const N: usize> {
    pub cells: [[Option<u8>; N]; N],
}

impl<const N: usize> PartialSudoku<N> {
    pub fn new(cells: [[Option<u8>; N]; N]) -> Self {
        Self {
            cells: cells.map(|row| row.map(|v| v.map(|n| n - 1))),
        }
    }
}

impl<const N: usize> Display for PartialSudoku<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.cells.iter().enumerate() {
            for cell in row {
                match cell {
                    Some(v) => write!(f, "{v}")?,
                    None => write!(f, "_")?,
                }
            }
            if y != N - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl<const N: usize> Index<(usize, usize)> for PartialSudoku<N> {
    type Output = Option<u8>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (y, x) = index;
        &self.cells[y][x]
    }
}

impl<const N: usize> From<[[Option<u8>; N]; N]> for PartialSudoku<N> {
    fn from(cells: [[Option<u8>; N]; N]) -> Self {
        Self { cells }
    }
}

impl<const N: usize> From<Sudoku<N>> for PartialSudoku<N> {
    fn from(sudoku: Sudoku<N>) -> Self {
        sudoku.cells.map(|row| row.map(Some)).into()
    }
}

impl<const N: usize> TryInto<Sudoku<N>> for PartialSudoku<N> {
    type Error = &'static str;

    fn try_into(self) -> Result<Sudoku<N>, Self::Error> {
        if self.cells.iter().flatten().any(|&cell| cell.is_none()) {
            return Err("Cannot convert to Sudoku: all cells are None");
        }

        Ok(self.cells.map(|row| row.map(|cell| cell.unwrap())).into())
    }
}
