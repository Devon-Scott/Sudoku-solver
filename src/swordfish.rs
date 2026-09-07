use crate::helpers::*;
use crate::types::*;

pub fn swordfish_elimination(board: &mut Board) -> bool {

    
    false
}

#[cfg(test)]
mod tests { 
    use super::*;

    fn mask(values: &[usize]) -> BitMask {
        let mut result = [false; 9];
        for &value in values {
            result[value - 1] = true;
        }
        result
    }

    fn candidate_board(b: bool) -> Board {
        std::array::from_fn(|_| {
            std::array::from_fn(|_| Cell::Candidates([b; 9]))
        })
    }
}