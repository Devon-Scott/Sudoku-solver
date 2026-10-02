use crate::helpers::*;
use crate::types::*;

pub fn chute_remote_pairs(board: &mut Board) -> bool {
    let mut change = false;

    
    change
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

    #[test]
    fn chute_remote_pairs_eliminates_row_chute() {
        let mut board = candidate_board(true);

        // Remote Pairs
        board[0][7] = Cell::Candidates(mask(&[4, 7]));
        board[2][0] = Cell::Candidates(mask(&[4, 7]));

        // Chute
        board[1][3] = Cell::Candidates(mask(&[4, 6, 8]));
        board[1][4] = Cell::Value(3);
        board[1][5] = Cell::Candidates(mask(&[4, 6, 8]));

        // Targets
        board[0][0] = Cell::Candidates(mask(&[1, 4, 7]));
        board[2][6] = Cell::Candidates(mask(&[4, 7, 8]));

        // Red Herring: don't touch this
        board[1][6] = Cell::Candidates(mask(&[4, 7, 8]));

        assert!(chute_remote_pairs(&mut board));

        assert!(board[0][0].candidates().unwrap()[3] == false);
        assert!(board[2][6].candidates().unwrap()[3] == false);

        assert!(board[1][6].candidates().unwrap()[3] == true);
    }

    #[test]
    fn chute_remote_pairs_eliminates_col_chute() {
        let mut board = candidate_board(true);

        // Remote Pairs
        board[7][0] = Cell::Candidates(mask(&[4, 7]));
        board[0][2] = Cell::Candidates(mask(&[4, 7]));

        // Chute
        board[3][1] = Cell::Candidates(mask(&[4, 6, 8]));
        board[4][1] = Cell::Value(3);
        board[5][1] = Cell::Candidates(mask(&[4, 6, 8]));

        // Targets
        board[0][0] = Cell::Candidates(mask(&[1, 4, 7]));
        board[6][2] = Cell::Candidates(mask(&[4, 7, 8]));

        // Red Herring: don't touch this
        board[6][1] = Cell::Candidates(mask(&[4, 7, 8]));

        assert!(chute_remote_pairs(&mut board));

        assert!(board[0][0].candidates().unwrap()[3] == false);
        assert!(board[6][2].candidates().unwrap()[3] == false);

        assert!(board[6][1].candidates().unwrap()[3] == true);
    }

    #[test]
    fn chute_remote_pairs_eliminates_using_value_as_evidence_col() {
        let mut board = candidate_board(true);

        // Remote Pairs
        board[2][5] = Cell::Candidates(mask(&[2, 8]));
        board[4][4] = Cell::Candidates(mask(&[2, 8]));

        // Chute
        board[6][3] = Cell::Value(5);
        board[7][3] = Cell::Candidates(mask(&[4, 6]));
        board[8][3] = Cell::Value(2);

        // Targets
        board[2][4] = Cell::Candidates(mask(&[2, 7, 8]));
        board[3][5] = Cell::Candidates(mask(&[2, 4, 8]));

        // Red Herring: don't touch this
        board[3][6] = Cell::Candidates(mask(&[1, 2, 7]));

        assert!(chute_remote_pairs(&mut board));

        assert!(board[2][4].candidates().unwrap()[1] == false);
        assert!(board[3][5].candidates().unwrap()[1] == false);

        assert!(board[6][1].candidates().unwrap()[1] == true);
    }

    #[test]
    fn chute_remote_pairs_eliminates_using_value_as_evidence_row() {
        let mut board = candidate_board(true);

        // Remote Pairs
        board[5][2] = Cell::Candidates(mask(&[2, 8]));
        board[4][4] = Cell::Candidates(mask(&[2, 8]));

        // Chute
        board[3][6] = Cell::Value(5);
        board[3][7] = Cell::Candidates(mask(&[4, 6]));
        board[3][8] = Cell::Value(2);

        // Targets
        board[4][2] = Cell::Candidates(mask(&[2, 7, 8]));
        board[5][3] = Cell::Candidates(mask(&[2, 4, 8]));

        // Red Herring: don't touch this
        board[6][3] = Cell::Candidates(mask(&[1, 2, 7]));

        assert!(chute_remote_pairs(&mut board));

        assert!(board[4][2].candidates().unwrap()[1] == false);
        assert!(board[5][3].candidates().unwrap()[1] == false);

        assert!(board[1][6].candidates().unwrap()[1] == true);
    }
}