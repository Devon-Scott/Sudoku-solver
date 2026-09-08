use crate::candidates::*;
use crate::helpers::*;
use crate::types::*;

pub fn solve_naked_singles(board: &mut Board) -> bool {
    let mut change = false;
    for row in 0..9 {
        for col in 0..9 {
            if let Cell::Candidates(bits) = board[row][col] {
                if bits.bits_set() == 1 {
                    let index = bits.iter().position(|bit| *bit == true).unwrap();
                    board[row][col] = Cell::Value((index + 1) as i16);
                    change = true;
                    eliminate_candidates(board);
                }
            }
        }
    }
    change
}

fn solve_hidden_singles_in_unit(set: &mut [Cell; 9]) {
    for value in 0..9 {
        let mut seen = false;
        let mut single = true;
        let mut idx = 10;
        for i in 0..9 {
            if let Cell::Candidates(mask) = set[i] {
                if mask[value] && !seen {
                    seen = true;
                    idx = i;
                } 
                else if mask[value] && seen {
                    single = false;
                }
            }            
        }
        if seen && single {
            set[idx] = Cell::Value((value as i16) + 1);
        }
    }
}

pub fn solve_hidden_singles(board: &mut Board) -> bool {
    let mut change = false;

    for mode in &UnitMode::LIST {
        for index in 0..9 {
            let mut candidate_set = 
                get_unit(board, index, *mode);
            solve_hidden_singles_in_unit(&mut candidate_set);
            if set_unit(board, index, &candidate_set, *mode) {
                eliminate_candidates(board);
                change = true;
            }
        }
    }

    change
}

pub fn solve_singles(board: &mut Board) -> bool{
    solve_hidden_singles(board) | solve_naked_singles(board)
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
    fn solves_a_hidden_single_in_a_row() {
        let mut board = candidate_board(false);
        board[3][1] = Cell::Candidates(mask(&[2, 5]));
        board[3][4] = Cell::Candidates(mask(&[2, 8]));
        board[3][7] = Cell::Candidates(mask(&[2, 5]));

        assert!(solve_hidden_singles(&mut board));
        assert!(matches!(board[3][4], Cell::Value(8)));
    }

    #[test]
    fn solves_a_hidden_single_in_a_column() {
        let mut board = candidate_board(false);
        board[1][6] = Cell::Candidates(mask(&[3, 4]));
        board[4][6] = Cell::Candidates(mask(&[3, 9]));
        board[7][6] = Cell::Candidates(mask(&[3, 4]));

        assert!(solve_hidden_singles(&mut board));
        assert!(matches!(board[4][6], Cell::Value(9)));
    }

    #[test]
    fn solves_hidden_single_at_correct_position_in_box() {
        let mut board = candidate_board(false);

        // Candidate 7 appears only at box-local index 1: global (0, 1).
        board[0][0] = Cell::Candidates(mask(&[2, 3]));
        board[0][1] = Cell::Candidates(mask(&[2, 7]));
        board[0][2] = Cell::Candidates(mask(&[2, 3]));

        solve_hidden_singles(&mut board);

        assert!(matches!(board[0][1], Cell::Value(7)));
        assert!(
            !matches!(board[1][0], Cell::Value(7)),
            "box index was transposed"
        );
    }
}
