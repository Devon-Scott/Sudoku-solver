mod box_line;
mod candidates;
mod helpers;
mod pairs;
mod parser;
mod singles;
mod subsets;
mod swordfish;
mod test_puzzles;
mod types;

use std::{io, env};
use std::time::Instant;

use crate::box_line::*;
use crate::candidates::*;
use crate::pairs::*;
use crate::parser::*;
use crate::singles::*;
use crate::subsets::*;
use crate::swordfish::*;
use crate::test_puzzles::*;
use crate::types::*;

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
        let grid: BasicGrid = NOT_FUN;
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
    
    let mut empty_count = board.iter()
        .flatten()
        .filter(|&&cell| matches!(cell, Cell::Empty))
        .count();
    println!("Input board ({empty_count} empty cells):");
    println!("{}", Grid(cells_to_grid(&board)));

    make_candidate_sets(&mut board);

    let start = Instant::now();
    let mut iter = 0;
    let mut c = eliminate_candidates(&mut board);
    while c {
        c = false;
        c |= eliminate_candidates(&mut board);
        c |= solve_naked_singles(&mut board);
        c |= eliminate_candidates(&mut board);
        c |= solve_hidden_singles(&mut board);
        c |= eliminate_candidates(&mut board);
        c |= determine_naked_doubles(&mut board);
        c |= determine_hidden_doubles(&mut board);
        c |= eliminate_pointing_sets(&mut board);
        c |= box_line_reduction(&mut board, helpers::UnitMode::Row);
        c |= box_line_reduction(&mut board, helpers::UnitMode::Column);
        c |= determine_naked_subsets(&mut board);
        if c {
            iter += 1;
        }
    }
    let duration = start.elapsed().as_micros();
    println!("Board after current algorithm");
    println!("{}", Grid(cells_to_grid(&board)));

    if verify(&cells_to_grid(&board)) {
        if iter == 1 {
            println!("Sudoku Solved in {duration} µs,\nusing {iter} iteration of constraint propagation!");
        }
        else {
            println!("Sudoku Solved in {duration} µs,\nusing {iter} iterations of constraint propagation!");
        }
    }
    else {
        empty_count = board.iter()
            .flatten()
            .filter(|&&cell| matches!(cell, Cell::Candidates(_)))
            .count();
        println!("Unable to solve. {empty_count} cells remain unsolved. Need more heuristics");
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

    fn assert_solution_still_possible(
        board: &Board,
        solution: &BasicGrid,
        phase: &str,
    ) {
        for row in 0..9 {
            for col in 0..9 {
                let expected = solution[row][col];

                match board[row][col] {
                    Cell::Value(actual) => {
                        assert_eq!(actual, expected, "wrong value after {phase}");
                    }
                    Cell::Candidates(bits) => {
                        assert!(
                            bits[(expected - 1) as usize],
                            "correct candidate {expected} removed from ({row}, {col}) after {phase}"
                        );
                    }
                    Cell::Empty => panic!("unexpected empty cell after {phase}"),
                }
            }
        }
    }

    #[test]
    fn full_integration_test() {
        let mut board = grid_to_cells(&NOT_FUN);
        make_candidate_sets(&mut board);
        eliminate_candidates(&mut board);
        assert_no_duplicate_values(&board, "initial candidate elimination");
        assert_no_empty_candidates(&board, "initial candidate elimination");
        assert_solution_still_possible(
            &board,
            &NOT_FUN_SOLUTION,
            "initial candidate elimination",
        );

        loop {
            let mut changed = false;

            changed |= solve_naked_singles(&mut board);
            assert_no_duplicate_values(&board, "naked singles");
            assert_no_empty_candidates(&board, "naked singles");
            assert_solution_still_possible(&board, &NOT_FUN_SOLUTION, "naked singles");

            changed |= eliminate_candidates(&mut board);
            assert_no_duplicate_values(&board, "elimination after naked singles");
            assert_no_empty_candidates(&board, "elimination after naked singles");
            assert_solution_still_possible(
                &board,
                &NOT_FUN_SOLUTION,
                "elimination after naked singles",
            );

            changed |= solve_hidden_singles(&mut board);
            assert_no_duplicate_values(&board, "hidden singles");
            assert_no_empty_candidates(&board, "hidden singles");
            assert_solution_still_possible(&board, &NOT_FUN_SOLUTION, "hidden singles");

            changed |= eliminate_candidates(&mut board);
            assert_no_duplicate_values(&board, "elimination after hidden singles");
            assert_no_empty_candidates(&board, "elimination after hidden singles");
            assert_solution_still_possible(
                &board,
                &NOT_FUN_SOLUTION,
                "elimination after hidden singles",
            );

            changed |= determine_naked_doubles(&mut board);
            assert_no_duplicate_values(&board, "determine naked doubles");
            assert_no_empty_candidates(&board, "determine naked doubles");
            assert_solution_still_possible(
                &board,
                &NOT_FUN_SOLUTION,
                "determine naked doubles",
            );

            changed |= determine_hidden_doubles(&mut board);
            assert_no_duplicate_values(&board, "determine hidden doubles");
            assert_no_empty_candidates(&board, "determine hidden doubles");
            assert_solution_still_possible(
                &board,
                &NOT_FUN_SOLUTION,
                "determine hidden doubles",
            );

            changed |= eliminate_pointing_sets(&mut board);
            assert_no_duplicate_values(&board, "Eliminate pointing sets");
            assert_no_empty_candidates(&board, "Eliminate pointing sets");
            assert_solution_still_possible(
                &board,
                &NOT_FUN_SOLUTION,
                "Eliminate pointing sets",
            );

            changed |= box_line_reduction(&mut board, helpers::UnitMode::Row);
            assert_no_duplicate_values(&board, "Box Line Reduction in Row");
            assert_no_empty_candidates(&board, "Box Line Reduction in Row");
            assert_solution_still_possible(
                &board,
                &NOT_FUN_SOLUTION,
                "Box Line Reduction in Row",
            );

            changed |= box_line_reduction(&mut board, helpers::UnitMode::Column);
            assert_no_duplicate_values(&board, "Box Line Reduction in Column");
            assert_no_empty_candidates(&board, "Box Line Reduction in Column");
            assert_solution_still_possible(
                &board,
                &NOT_FUN_SOLUTION,
                "Box Line Reduction in Column",
            );

            changed |= determine_naked_subsets(&mut board);
            assert_no_duplicate_values(&board, "Determine Naked Subsets");
            assert_no_empty_candidates(&board, "Determine Naked Subsets");
            assert_solution_still_possible(
                &board,
                &NOT_FUN_SOLUTION,
                "Determine Naked Subsets",
            );

            if !changed {
                break;
            }
        }
    }
}
