use std::fmt::Display;

/// 4x4 Sudoku grid bitboard representation.
/// Each cell is encoded using 4 bits, where each bit acts as a flag indicating
/// whether the cell can take the corresponding value. For example:
/// - 0b_1100 -> cell can have "3" or "2"
/// - 0b_0100 -> cell can only have "2"
/// - 0b_0000 -> cell can have no value, that means given problem to be unsolveable.
///
/// Cells are arranged in a order from LSB to MSB.
///
/// +---------+-----+---------+---------+
/// | cell 15 | ... |  cell 1 |  cell 0 |
/// +---------+-----+---------+---------+
/// | 3|2|1|0 | ... | 3|2|1|0 | 3|2|1|0 |
/// +---------+-----+---------+---------+
/// MSB ----------------------------> LSB
///
/// Rows, columns, cells are indexed as shown in the following table.
/// Boxes are indexed in the similar way.
///
/// row/col|  3  2  1  0
/// -------+------------
///      3 | 15 14 13 12
///      2 | 11 10 09 08
///      1 | 07 06 05 04
///      0 | 03 02 01 00
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grid {
    value: u64,
}

impl Grid {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn full() -> Self {
        Self::new(0xffff_ffff_ffff_ffff)
    }

    pub fn get_cell(&self, index: usize) -> u64 {
        (self.value >> (index * 4)) & 0xf
    }

    pub fn mask_cell(&mut self, index: usize, mask: u64) {
        let mask = !((!mask & 0xf) << (index * 4));
        self.value &= mask
    }

    pub fn unmask_cell(&mut self, index: usize, mask: u64) {
        let mask = (mask & 0xf) << (index * 4);
        self.value |= mask
    }

    pub fn mask_row(&mut self, index: usize, mask: u64) {
        let mask = (mask << 12) | (mask << 8) | (mask << 4) | mask;
        let mask = !((!mask & 0xffff) << (index * 16));
        self.value &= mask
    }

    pub fn mask_column(&mut self, index: usize, mask: u64) {
        let mask = (mask << 48) | (mask << 32) | (mask << 16) | mask;
        let mask = !((!mask & 0x000f_000f_000f_000f) << (index * 4));
        self.value &= mask;
    }

    pub fn mask_block(&mut self, index: usize, mask: u64) {
        let (bx, by) = (index % 2, index / 2);
        let mask = (mask << 20) | (mask << 16) | (mask << 4) | mask;
        let mask = !((!mask & 0x00ff_00ff) << (bx * 8 + by * 32));
        self.value &= mask;
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+----+----+----+----+")?;
        for y in 0..4 {
            write!(f, "|")?;
            for x in 0..4 {
                let cell = self.get_cell(x + y * 4);

                write!(f, "{}", if cell & 0b1000 != 0 { "4" } else { " " })?;
                write!(f, "{}", if cell & 0b0100 != 0 { "3" } else { " " })?;
                write!(f, "{}", if cell & 0b0010 != 0 { "2" } else { " " })?;
                write!(f, "{}", if cell & 0b0001 != 0 { "1" } else { " " })?;

                write!(f, "|")?;
                if x == 3 {
                    writeln!(f)?;
                }
            }
            write!(f, "+----+----+----+----+")?;
            if y != 3 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cell() {
        let grid = Grid::new(0xfedc_ba98_7654_3210);

        assert_eq!(grid.get_cell(0), 0b0000);
        assert_eq!(grid.get_cell(1), 0b0001);
        assert_eq!(grid.get_cell(2), 0b0010);
        assert_eq!(grid.get_cell(3), 0b0011);
        assert_eq!(grid.get_cell(4), 0b0100);
        assert_eq!(grid.get_cell(5), 0b0101);
        assert_eq!(grid.get_cell(6), 0b0110);
        assert_eq!(grid.get_cell(7), 0b0111);
        assert_eq!(grid.get_cell(8), 0b1000);
        assert_eq!(grid.get_cell(9), 0b1001);
        assert_eq!(grid.get_cell(10), 0b1010);
        assert_eq!(grid.get_cell(11), 0b1011);
        assert_eq!(grid.get_cell(12), 0b1100);
        assert_eq!(grid.get_cell(13), 0b1101);
        assert_eq!(grid.get_cell(14), 0b1110);
        assert_eq!(grid.get_cell(15), 0b1111);
    }

    #[test]
    fn test_mask_cell() {
        let mut grid = Grid::new(0xfedc_ba98_7654_3210);

        grid.mask_cell(7, 0b1010);

        assert_eq!(grid.value, 0xfedc_ba98_2654_3210);
        //                                 ^ 0b0111 & 0b1010 = 0b0010
    }

    #[test]
    fn test_mask_cell_0xf() {
        let mut grid = Grid::new(0xfedc_ba98_7654_3210);

        for i in 0..16 {
            grid.mask_cell(i, 0b1111);
        }

        assert_eq!(grid.value, 0xfedc_ba98_7654_3210);
    }

    #[test]
    fn test_mask_cell_0x0() {
        let mut grid = Grid::new(0xfedc_ba98_7654_3210);

        for i in 0..16 {
            grid.mask_cell(i, 0b0000);
        }

        assert_eq!(grid.value, 0x0000_0000_0000_0000);
    }

    #[test]
    fn test_unmask_cell() {
        let mut grid = Grid::new(0xfedc_ba98_7654_3210);

        grid.unmask_cell(8, 0b1010);

        assert_eq!(grid.value, 0xfedc_ba9a_7654_3210);
        //                               ^ 0b1000 | 0b1010 = 0b1010
    }

    #[test]
    fn test_unmask_cell_0xf() {
        let mut grid = Grid::new(0xfedc_ba98_7654_3210);

        for i in 0..16 {
            grid.unmask_cell(i, 0b1111);
        }

        assert_eq!(grid.value, 0xffff_ffff_ffff_ffff);
    }

    #[test]
    fn test_unmask_cell_0x0() {
        let mut grid = Grid::new(0xfedc_ba98_7654_3210);

        for i in 0..16 {
            grid.unmask_cell(i, 0b0000);
        }

        assert_eq!(grid.value, 0xfedc_ba98_7654_3210);
    }

    #[test]
    fn test_mask_row() {
        let mut grid = Grid::new(0xfedc_ba98_7654_3210);

        grid.mask_row(3, 0b1010);

        assert_eq!(grid.value, 0xaa88_ba98_7654_3210);
        //                       ^ 0xfedc & 0xaaaa = 0xaa88
    }

    #[test]
    fn test_mask_row_0xf() {
        let mut grid = Grid::new(0xfedc_ba98_7654_3210);

        for i in 0..4 {
            grid.mask_row(i, 0b1111);
        }

        assert_eq!(grid.value, 0xfedc_ba98_7654_3210);
    }

    #[test]
    fn test_mask_row_0x0() {
        let mut grid = Grid::new(0xfedc_ba98_7654_3210);

        for i in 0..4 {
            grid.mask_row(i, 0b0000);
        }

        assert_eq!(grid.value, 0x0000_0000_0000_0000);
    }

    #[test]
    fn test_mask_column() {
        let mut grid = Grid::new(0xfedc_ba98_7654_3210);

        grid.mask_column(3, 0b1010);

        assert_eq!(grid.value, 0xaedc_aa98_2654_2210);
        //                       ^    ^    ^    ^
        //                       |    |    |    0b0110 & 0b1010 = 0b0010
        //                       |    |    0b0111 & 0b1010 = 0b0010
        //                       |    0b1011 & 0b1010 = 0b1010
        //                       0b1111 & 0b1010 = 0b1010
    }

    #[test]
    fn test_mask_column_0xf() {
        let mut grid = Grid::new(0xfedc_ba98_7654_3210);

        for i in 0..4 {
            grid.mask_column(i, 0b1111);
        }

        assert_eq!(grid.value, 0xfedc_ba98_7654_3210);
    }

    #[test]
    fn test_mask_column_0x0() {
        let mut grid = Grid::new(0xfedc_ba98_7654_3210);

        for i in 0..4 {
            grid.mask_column(i, 0b0000);
        }

        assert_eq!(grid.value, 0x0000_0000_0000_0000);
    }

    #[test]
    fn test_mask_block() {
        let mut grid = Grid::new(0xfedc_ba98_7654_3210);

        grid.mask_block(3, 0b1010);

        assert_eq!(grid.value, 0xaadc_aa98_7654_3210);
        //                       ^^   ^^
    }

    #[test]
    fn test_mask_block_0xf() {
        let mut grid = Grid::new(0xfedc_ba98_7654_3210);
        for i in 0..4 {
            grid.mask_block(i, 0b1111);
        }

        assert_eq!(grid.value, 0xfedc_ba98_7654_3210);
    }

    #[test]
    fn test_mask_block_0x0() {
        let mut grid = Grid::new(0xfedc_ba98_7654_3210);
        for i in 0..4 {
            grid.mask_block(i, 0b0000);
        }

        assert_eq!(grid.value, 0x0000_0000_0000_0000);
    }
}
