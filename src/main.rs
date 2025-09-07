use std::io::Read;

use sudoku::SudokuProblem;

fn parse_grid(s: &str) -> [[Option<u8>; 4]; 4] {
    let split: Vec<Vec<_>> = s.split('\n').map(|line| line.chars().collect()).collect();

    let mut grid = [[None; 4]; 4];
    for y in 0..4 {
        for x in 0..4 {
            grid[y][x] = match split[y][x] {
                '1' => Some(1),
                '2' => Some(2),
                '3' => Some(3),
                '4' => Some(4),
                _ => None,
            }
        }
    }

    grid
}

fn main() {
    // let problem = SudokuProblem::new([
    //     [Some(1), None, Some(3), Some(4)],
    //     [None, Some(4), None, Some(1)],
    //     [None, None, None, None],
    //     [Some(4), None, Some(1), None],
    // ]);

    let mut stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).unwrap();

    let problem = SudokuProblem::new(parse_grid(&buffer));

    println!("Problem:");
    println!("{problem}");

    println!();

    if let Some(answer) = sudoku::solver::solve(problem) {
        let answer = SudokuProblem::from_completed(answer);
        println!("Answer:");
        println!("{answer}")
    } else {
        println!("The problem could has no solution.")
    }
}
