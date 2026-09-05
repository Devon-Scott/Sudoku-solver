use crate::helpers::*;
use crate::types::*;

// A generalization of naked pairs, where k cells collectively
// contain only k possible values (here k goes up to 4)
pub fn determine_naked_subsets(board: &mut Board) -> bool {
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

    fn candidate_board() -> Board {
        std::array::from_fn(|_| {
            std::array::from_fn(|_| Cell::Candidates([false; 9]))
        })
    }

    #[test]
    fn determine_naked_subsets_isolates_triple_in_row() {
        let mut board = candidate_board();
        board[3][1] = Cell::Candidates(mask(&[1, 2]));
        board[3][4] = Cell::Candidates(mask(&[2, 3]));
        board[3][7] = Cell::Candidates(mask(&[1, 3]));
        board[3][8] = Cell::Candidates(mask(&[1, 2, 3, 6, 9]));

        assert!(determine_naked_subsets(&mut board));
        let correct_mask = mask(&[6, 9]);
        assert!(matches!(board[3][8], Cell::Candidates(actual) if actual == correct_mask));
    }
}