use crate::types::*;

#[derive(PartialEq)]
pub enum UnitMode {
    Row,
    Column,
    Box 
}

// Returns a vector of bitmasks and their corresponding index 
// in the row, column, or box that (row, col) belongs too
// Boxes are indexed left to right, then top to bottom
pub fn get_unit_candidate_masks(board: &Board, row: usize, col: usize, mode: UnitMode)
-> Vec<(usize, BitMask)> {
    let mut candidate_set: Vec<(usize, BitMask)> = Vec::new();
    match mode {
        UnitMode::Row => {
            for c in 0..9 {
                if let Cell::Candidates(bits) = board[row][c] {
                    candidate_set.push((c, bits));
                }
            }
        }
        UnitMode::Column => {
            for r in 0..9 {
                if let Cell::Candidates(bits) = board[r][col] {
                    candidate_set.push((r, bits));
                }
            }
        }
        UnitMode::Box => {
            // Truncated division: e.g. row in [3, 4, 5] -> r = 3
            let row_start = (row / 3) * 3;
            let col_start = (col / 3) * 3;
            let mut idx = 0;
            for r in 0..3 {
                for c in 0..3 {
                    let r_idx = r + row_start;
                    let c_idx = c + col_start;
                    if let Cell::Candidates(bits) = board[r_idx][c_idx] {
                        candidate_set.push((idx, bits));
                    }
                    idx += 1;
                }
            }
        }
    }
    candidate_set
}