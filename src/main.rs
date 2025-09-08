use std::io::Read;

use sudoku::PartialSudoku;

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
    // let problem = PartialSudoku::<4>::new([
    //     [Some(1), None, Some(3), Some(4)],
    //     [None, Some(4), None, Some(1)],
    //     [None, None, None, None],
    //     [Some(4), None, Some(1), None],
    // ]);
    // println!("Problem:");
    // println!("{problem}");

    let mut stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).unwrap();

    let problem = PartialSudoku::new(parse_grid(&buffer));

    if let Some(answer) = sudoku::solve(problem) {
        println!("{answer}")
    } else {
        println!("no solution")
    }
}
