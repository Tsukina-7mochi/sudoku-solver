use std::collections::BinaryHeap;

use super::SudokuProblem;

mod cell_node;
mod grid;

use cell_node::CellNode;
use grid::Grid;

fn adjacent_cells(index: usize) -> impl Iterator<Item = usize> {
    let (x, y) = (index % 4, index / 4);
    let (bx, by) = (x / 2, y / 2);

    let col_iter = (0..4).map(move |i| x + i * 4);
    let row_iter = (0..4).map(move |i| i + y * 4);
    let block_iter = (0..2).map(move |i| {
        let (ix, iy) = (i % 2, i / 2);
        (bx * 2 + ix) + (by * 2 + iy) * 4
    });

    col_iter.chain(row_iter).chain(block_iter)
}

pub fn solve(sudoku: SudokuProblem) -> Option<[[u8; 4]; 4]> {
    let mut grid = Grid::full();
    let mut queue = BinaryHeap::new();
    let mut settled = [false; 16];

    // initialize the grid with the given sudoku puzzle
    for i in 0..16 {
        let (x, y) = (i % 4, i / 4);
        if let Some(value) = sudoku[(y, x)] {
            grid.mask_cell(i, 1 << value);
            queue.push(CellNode::new(i, 1));
        }
    }

    loop {
        while let Some(node) = queue.pop() {
            let idx = node.index;

            if settled[idx] {
                continue;
            }
            settled[idx] = true;

            let (x, y) = (idx % 4, idx / 4);
            let block = (y / 2) * 2 + (x / 2);
            let mask = {
                let value = grid.get_cell(idx);

                match value.count_ones() {
                    // the cell can not take any value, that means the puzzle is unsolvable
                    0 => return None,

                    // the cell can only take one value, so we use that value as is
                    1 => !value & 0b1111,

                    // the cell can take multiple values, so we pick the lowest one
                    _ => {
                        let value = value.trailing_zeros();
                        !(1 << value) & 0b1111
                    }
                }
            };

            // println!("{grid}");
            // println!("@({x}, {y}) mask: 0b{mask:04b}");

            // mask and enqueue adjacent cells
            grid.mask_column(x, mask);
            grid.mask_row(y, mask);
            grid.mask_block(block, mask);
            grid.unmask_cell(idx, !mask);

            for adj in adjacent_cells(idx) {
                if settled[adj] {
                    continue;
                }

                let popcount = grid.get_cell(adj).count_ones() as usize;
                queue.push(CellNode::new(adj, popcount));
            }
        }

        // we have settled all cells we can, check if there are any unsettled cells left
        let first_unsettled = settled.iter().position(|&b| !b);
        if let Some(idx) = first_unsettled {
            let popcount = grid.get_cell(idx).count_ones() as usize;
            queue.push(CellNode::new(idx, popcount));
        } else {
            break;
        }
    }

    let mut result = [[0u8; 4]; 4];
    for i in 0..16 {
        let (x, y) = (i % 4, i / 4);
        let value = grid.get_cell(i);
        if value == 0 {
            return None;
        }

        let value = value.trailing_zeros() as u8;
        result[y][x] = value + 1;
    }

    Some(result)
}
