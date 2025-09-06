use sudoku::sudoku::SudokuProblem;

fn main() {
    let sudoku = SudokuProblem::new([
        [Some(1), None, Some(3), Some(4)],
        [None, Some(4), None, Some(1)],
        [None, None, None, None],
        [Some(4), None, Some(1), None],
    ]);
    println!("{sudoku}");
}
