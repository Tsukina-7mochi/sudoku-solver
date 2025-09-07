use std::fmt::Display;
use std::ops::Index;

pub struct SudokuProblem {
    grid: [[Option<u8>; 4]; 4],
}

impl SudokuProblem {
    pub fn new(grid: [[Option<u8>; 4]; 4]) -> Self {
        Self {
            grid: grid.map(|row| row.map(|cell| cell.map(|v| v - 1))),
        }
    }

    pub fn from_completed(grid: [[u8; 4]; 4]) -> Self {
        Self {
            grid: grid.map(|row| row.map(|cell| Some(cell - 1))),
        }
    }
}

impl Display for SudokuProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..4 {
            for x in 0..4 {
                match self[(y, x)] {
                    Some(v) => write!(f, "{}", v + 1)?,
                    None => write!(f, "_")?,
                }
            }
            if y != 3 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Index<(usize, usize)> for SudokuProblem {
    type Output = Option<u8>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (y, x) = index;
        &self.grid[y][x]
    }
}
