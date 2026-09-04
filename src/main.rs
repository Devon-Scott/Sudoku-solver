mod candidates;
mod pairs;
mod parser;
mod singles;
mod types;

use std::{io, env};

use crate::candidates::*;
use crate::pairs::*;
use crate::parser::*;
use crate::singles::*;
use crate::types::*;

// Sampled from the Sudoku app on my phone
const TEST_GRID: BasicGrid = [
    [0, 0, 0, 0, 0, 8, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 9, 0, 0],
    [3, 0, 4, 9, 0, 0, 8, 2, 0],
    [7, 0, 0, 0, 0, 2, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 2, 0, 0],
    [0, 0, 9, 0, 1, 6, 0, 7, 0],
    [0, 0, 3, 0, 6, 0, 0, 0, 1],
    [0, 6, 1, 0, 8, 3, 0, 4, 0],
    [4, 0, 0, 5, 0, 0, 0, 6, 0]];

// Sourced from https://sandiway.arizona.edu/sudoku/examples.html
// const NOT_FUN: BasicGrid = [
//     [0, 2, 0, 0, 0, 0, 0, 0, 0],
//     [0, 0, 0, 6, 0, 0, 0, 0, 3],
//     [0, 7, 4, 0, 8, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0, 3, 0, 0, 2],
//     [0, 8, 0, 0, 4, 0, 0, 1, 0],
//     [6, 0, 0, 5, 0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 1, 0, 7, 8, 0],
//     [5, 0, 0, 0, 0, 9, 0, 0, 0],
//     [0, 0, 0, 0, 0, 0, 0, 4, 0]];

// const SOLVED_GRID: BasicGrid = [
//     [1, 2, 3, 4, 5, 6, 7, 8, 9],
//     [4, 5, 6, 7, 8, 9, 1, 2, 3],
//     [7, 8, 9, 1, 2, 3, 4, 5, 6],
//     [2, 3, 4, 5, 6, 7, 8, 9, 1],
//     [5, 6, 7, 8, 9, 1, 2, 3, 4],
//     [8, 9, 1, 2, 3, 4, 5, 6, 7],
//     [3, 4, 5, 6, 7, 8, 9, 1, 2],
//     [6, 7, 8, 9, 1, 2, 3, 4, 5],
//     [9, 1, 2, 3, 4, 5, 6, 7, 8]
// ];

fn verify(grid: &BasicGrid) -> bool {
    for row in 0..9 {
        // Maybe use a bit vector and logical operators?
        let mut row_checks: [bool; 9] = [false; 9];
        for col in 0..9 {
            let num: usize = grid[row][col] as usize;
            if num == 0 {
                return false
            }
            row_checks[num - 1] = true
        }
        for i in 0..9{
            if row_checks[i] == false {
                return false
            }
        }
    }

    for col in 0..9 {
        let mut col_checks: [bool; 9] = [false; 9];
        for row in 0..9 {
            let num: usize = grid[row][col] as usize;
            if num == 0 {
                return false
            }
            col_checks[num - 1] = true
        }
        for i in 0..9{
            if col_checks[i] == false {
                return false
            }
        }
    }

    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut box_checks = [false; 9];
            for row in 0..3 {
                for col in 0..3 {
                    let r: usize = box_row * 3 + row;
                    let c: usize = box_col * 3 + col;
                    let num: usize = grid[r][c] as usize;
                    if num == 0 {
                        return false
                    }
                    box_checks[num - 1] = true;
                }
            }
            for i in 0..9{
                if box_checks[i] == false {
                    return false
                }
            }
        }
    }
    true
}

fn make_candidate_sets(board: &mut Board) {
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == Cell::Empty {
                board[row][col] = Cell::Candidates([true; 9])
            }
        }
    }
}
fn grid_to_cells(grid: &BasicGrid) -> Board {
    grid.map(|row| {
        row.map(|val| match val {
            0 => Cell::Empty,
            n => Cell::Value(n)
        })
    })
}

fn cells_to_grid(board: &Board) -> BasicGrid {
    let mut result: BasicGrid = [[0; 9]; 9];
    for row in 0..9 {
        for col in 0..9 {
            result[row][col] = match &board[row][col] {
                Cell::Value(num) => *num,
                _ => 0
            };
        }
    }
    result
}

fn main() -> Result<(), io::Error>{
    let args: Vec<String> = env::args().collect();

    let mut board = if args.len() > 1 && &args[1] == "--test" {
        let grid: BasicGrid = TEST_GRID;
        let board: Board = grid_to_cells(&grid);
        board
    }
    else {
        let mut parser = Parser::new();
        let Some(board) = parser.parse()? else {
            return Ok(());
        };
        board
    };
    
    println!("Input board:");
    println!("{}", Grid(cells_to_grid(&board)));

    make_candidate_sets(&mut board);

    let mut c = eliminate_candidates(&mut board);
    while c {
        c = false;
        c |= eliminate_candidates(&mut board);
        c |= solve_naked_singles(&mut board);
        c |= eliminate_candidates(&mut board);
        c |= solve_hidden_singles(&mut board);
        c |= eliminate_candidates(&mut board);
        c |= determine_naked_doubles(&mut board);
    }
    println!("Board after current algorithm");
    println!("{}", Grid(cells_to_grid(&board)));

    // Candidates([false, false, false, false, false, false, false, false, false])
    if verify(&cells_to_grid(&board)) {
        println!("Sudoku Solved!");
    }
    else {
        println!("Unable to solve, need more heuristics");
    }
    return Ok(())

}

#[cfg(test)]
mod tests {
    
    use super::*;

    fn assert_no_duplicate_values(board: &Board, phase: &str) {
        for row in 0..9 {
            let mut seen = [false; 9];
            for col in 0..9 {
                if let Cell::Value(value) = board[row][col] {
                    let idx = (value - 1) as usize;
                    assert!(!seen[idx], "duplicate {value} in row {row} after {phase}");
                    seen[idx] = true;
                }
            }
        }

        for col in 0..9 {
            let mut seen = [false; 9];
            for row in 0..9 {
                if let Cell::Value(value) = board[row][col] {
                    let idx = (value - 1) as usize;
                    assert!(!seen[idx], "duplicate {value} in column {col} after {phase}");
                    seen[idx] = true;
                }
            }
        }

        for box_row in 0..3 {
            for box_col in 0..3 {
                let mut seen = [false; 9];
                for row_offset in 0..3 {
                    for col_offset in 0..3 {
                        let row = box_row * 3 + row_offset;
                        let col = box_col * 3 + col_offset;
                        if let Cell::Value(value) = board[row][col] {
                            let idx = (value - 1) as usize;
                            assert!(
                                !seen[idx],
                                "duplicate {value} in box ({box_row}, {box_col}) after {phase}"
                            );
                            seen[idx] = true;
                        }
                    }
                }
            }
        }
    }

    fn assert_no_empty_candidates(board: &Board, phase: &str) {
        for row in 0..9 {
            for col in 0..9 {
                if let Cell::Candidates(bits) = &board[row][col] {
                    assert!(
                        bits.bits_set() > 0,
                        "cell ({row}, {col}) has no remaining candidates after {phase}: {:?}\n{}",
                        board[row][col], Grid(cells_to_grid(board))
                    );
                }
            }
        }
    }

    #[test]
    fn every_solver_phase_preserves_sudoku_uniqueness() {
        let mut board = grid_to_cells(&TEST_GRID);
        make_candidate_sets(&mut board);
        eliminate_candidates(&mut board);
        assert_no_duplicate_values(&board, "initial candidate elimination");
        assert_no_empty_candidates(&board, "initial candidate elimination");

        loop {
            let mut changed = false;

            changed |= solve_naked_singles(&mut board);
            assert_no_duplicate_values(&board, "naked singles");
            assert_no_empty_candidates(&board, "naked singles");

            changed |= eliminate_candidates(&mut board);
            assert_no_duplicate_values(&board, "elimination after naked singles");
            assert_no_empty_candidates(&board, "elimination after naked singles");

            changed |= solve_hidden_singles(&mut board);
            assert_no_duplicate_values(&board, "hidden singles");
            assert_no_empty_candidates(&board, "hidden singles");

            changed |= eliminate_candidates(&mut board);
            assert_no_duplicate_values(&board, "elimination after hidden singles");
            assert_no_empty_candidates(&board, "elimination after hidden singles");

            changed |= determine_naked_doubles(&mut board);
            assert_no_duplicate_values(&board, "determine naked doubles");
            assert_no_empty_candidates(&board, "determine naked doubles");

            if !changed {
                break;
            }
        }
    }
}
