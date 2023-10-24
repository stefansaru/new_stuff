extern crate rand;

use rand::Rng;
use std::fmt;

// Constants
const SIZE: usize = 9;
const BOX_SIZE: usize = 3;

// Struct to represent a Sudoku puzzle
struct Sudoku {
    grid: [[u8; SIZE]; SIZE],
}

impl Sudoku {
    fn new() -> Sudoku {
        Sudoku { grid: [[0; SIZE]; SIZE] }
    }

    fn is_valid_move(&self, row: usize, col: usize, num: u8) -> bool {
        for i in 0..SIZE {
            if self.grid[row][i] == num || self.grid[i][col] == num {
                return false;
            }
        }

        let box_row = (row / BOX_SIZE) * BOX_SIZE;
        let box_col = (col / BOX_SIZE) * BOX_SIZE;

        for i in 0..BOX_SIZE {
            for j in 0..BOX_SIZE {
                if self.grid[box_row + i][box_col + j] == num {
                    return false;
                }
            }
        }

        true
    }

    fn solve(&mut self) -> bool {
        for row in 0..SIZE {
            for col in 0..SIZE {
                if self.grid[row][col] == 0 {
                    for num in 1..=SIZE as u8 {
                        if self.is_valid_move(row, col, num) {
                            self.grid[row][col] = num;
                            if self.solve() {
                                return true;
                            }
                            self.grid[row][col] = 0;
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        self.solve();
        for _ in 0..SIZE * SIZE {
            let mut row: usize;
            let mut col: usize;
            loop {
                row = rng.gen_range(0..SIZE);
                col = rng.gen_range(0..SIZE);
                if self.grid[row][col] != 0 {
                    self.grid[row][col] = 0;
                    break;
                }
            }
        }
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.grid.iter() {
            for num in row.iter() {
                write!(f, "{} ", num)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut puzzle = Sudoku::new();
    puzzle.generate();
    println!("Random Sudoku Puzzle:\n{}", puzzle);
}