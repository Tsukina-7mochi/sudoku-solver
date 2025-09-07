use sudoku::SudokuProblem;

fn main() {
    let problem = SudokuProblem::new([
        [Some(1), None, Some(3), Some(4)],
        [None, Some(4), None, Some(1)],
        [None, None, None, None],
        [Some(4), None, Some(1), None],
    ]);

    println!("{problem}");

    if let Some(answer) = sudoku::solver::solve(problem) {
        let answer = SudokuProblem::from_completed(answer);
        println!("{answer}")
    } else {
        println!("The problem could has no solution.")
    }
}
