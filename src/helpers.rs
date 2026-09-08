use crate::types::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnitMode {
    Row,
    Column,
    Box 
}

impl UnitMode {
    pub const LIST: [UnitMode; 3] = [
        UnitMode::Row,
        UnitMode::Column,
        UnitMode::Box
    ];
}

// Returns a vector of bitmasks and their corresponding index 
// in the indexed unit (row, col, or box) that they belong to
// Boxes are indexed left to right, then top to bottom
pub fn get_unit_candidate_masks(board: &Board, index: usize, mode: UnitMode)
-> Vec<(usize, BitMask)> {
    debug_assert!(index < 9);
    let mut candidate_set: Vec<(usize, BitMask)> = Vec::new();
    match mode {
        UnitMode::Row => {
            for c in 0..9 {
                if let Cell::Candidates(bits) = board[index][c] {
                    candidate_set.push((c, bits));
                }
            }
        }
        UnitMode::Column => {
            for r in 0..9 {
                if let Cell::Candidates(bits) = board[r][index] {
                    candidate_set.push((r, bits));
                }
            }
        }
        UnitMode::Box => {
            // Truncated division: e.g. row in [3, 4, 5] -> r = 3
            let row_start = (index / 3) * 3;
            let col_start = (index % 3) * 3;
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

pub fn get_unit(board: &Board, index: usize, mode: UnitMode) -> [Cell; 9] {
    debug_assert!(index < 9);
    let mut result = [Cell::Empty; 9];
    match mode {
        UnitMode::Row => {
            for c in 0..9 {
                result[c] = board[index][c];
            }
        }
        UnitMode::Column => {
            for r in 0..9 {
                result[r] = board[r][index];
            }
        }
        UnitMode::Box => {
            let row_start = (index / 3) * 3;
            let col_start = (index % 3) * 3;
            let mut idx = 0;
            for r in 0..3 {
                for c in 0..3 {
                    let r_idx = r + row_start;
                    let c_idx = c + col_start;
                    result[idx] = board[r_idx][c_idx];
                    idx += 1;
                }
            }
        }
    }
    result
}

// The number of masks, and their indices, should equal the numbers returned 
// from a corresponding call to get_unit_candidate_masks
// Each index in (index, bitmask) is relative to the unit being referenced
pub fn set_unit_candidate_masks(board: &mut Board, index: usize, masks: &[(usize, BitMask)], mode: UnitMode) -> bool {
    debug_assert!(masks.len() <= 9);
    debug_assert!(masks.iter().all(|(unit_index, _)| *unit_index < 9));
    debug_assert!(index < 9);
    let mut change = false;
    match mode {
        UnitMode::Row => {
            for (col, mask) in masks {
                if let Cell::Candidates(bits) = &mut board[index][*col] {
                    if bits != mask {
                        board[index][*col] = Cell::Candidates(*mask);
                        change = true;
                    }
                }
                else {
                    panic!("Row Candidate indexes did not match a call to get_unit_candidate_masks")
                }
            }
        }
        UnitMode::Column => {
            for (row, mask) in masks {
                if let Cell::Candidates(bits) = &mut board[*row][index] {
                    if bits != mask {
                        board[*row][index] = Cell::Candidates(*mask);
                        change = true;
                    }
                }
                else {
                    panic!("Column Candidate indexes did not match a call to get_unit_candidate_masks")
                }
            }
        }
        UnitMode::Box => {
            // Truncated division: e.g. row in [3, 4, 5] -> r = 3
            let row_start = (index / 3) * 3;
            let col_start = (index % 3) * 3;
            for (idx, mask) in masks {
                let row = row_start + (idx / 3);
                let col = col_start + (idx % 3);
                if let Cell::Candidates(bits) = &mut board[row][col] {
                    if bits != mask {
                        board[row][col] = Cell::Candidates(*mask);
                        change = true;
                    }
                }
                else {
                    panic!("Box Candidate indexes did not match a call to get_unit_candidate_masks")
                }
            }
        }
    }
    change
}

pub fn set_unit(board: &mut Board, index: usize, masks: &[Cell; 9], mode: UnitMode) -> bool {
    debug_assert!(index < 9);
    let mut change = false;
    match mode {
        UnitMode::Row => {
            for col in 0..9 {
                if board[index][col] != masks[col] {
                    board[index][col] = masks[col];
                    change = true;
                }
            }
        }
        UnitMode::Column => {
            for row in 0..9 {
                if board[row][index] != masks[row] {
                    board[row][index] = masks[row];
                    change = true;
                }
            }
        }
        UnitMode::Box => {
            // Truncated division: e.g. row in [3, 4, 5] -> r = 3
            let row_start = (index / 3) * 3;
            let col_start = (index % 3) * 3;
            for idx in 0..9 {
                let row = row_start + (idx / 3);
                let col = col_start + (idx % 3);
                if board[row][col] != masks[idx] {
                    board[row][col] = masks[idx];
                    change = true;
                }
            }
        }
    }
    change
}