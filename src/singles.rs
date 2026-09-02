use crate::types::*;

pub fn get_index_of_unique_candidate(mask: &BitMask) -> Option<usize> {
    let mut idx: Option<usize> = None;
    for i in 0..9 {
        if mask[i]{
            if idx != None  {
                return None
            }
            idx = Some(i);
        }
    }
    idx
}

pub fn solve_naked_singles(board: &mut Board) -> bool {
    let mut change = false;
    for row in 0..9 {
        for col in 0..9 {
            if let Cell::Candidates(bits) = board[row][col] {
                let idx = get_index_of_unique_candidate(&bits);
                if let Some(num) = idx {
                    board[row][col] = Cell::Value((num + 1) as i16);
                    change = true;
                }
            }
        }
    }
    change
}