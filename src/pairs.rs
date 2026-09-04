use crate::helpers::*;
use crate::types::*;

fn isolate_naked_doubles_from_masks(candidate_set: &Vec<(usize, BitMask)>) -> Vec<(usize, usize, BitMask)>{
    let mut result: Vec<(usize, usize, BitMask)> = Vec::new();
    for (i, bits_i) in candidate_set {
        for (j, bits_j) in candidate_set {
            if i != j && bits_i == bits_j && bits_i.bits_set() == 2{
                result.push((*i, *j, *bits_i));
            }
        }
    }
    result
}

pub fn determine_naked_doubles(board: &mut Board) -> bool {
    let mut change = false;

    // For row
    for row in 0..9 {
        let candidate_set = 
            get_unit_candidate_masks(board, row, 0, UnitMode::Row);
        let pair_mask = 
            isolate_naked_doubles_from_masks(&candidate_set);

        for (i, j, mask) in pair_mask {
            for col in 0..9 {
                if col == i || col == j {
                    continue;
                }

                if let Cell::Candidates(ref mut bits) = board[row][col] {
                    for val_idx in 0..9 {
                        if mask[val_idx] && bits[val_idx] {
                            bits[val_idx] = false;
                            change = true;
                        }
                    }
                }
            }   
        }
    }

    // for col
    for col in 0..9 {
        let candidate_set= 
            get_unit_candidate_masks(board, 0, col, UnitMode::Column);
        let pair_mask = 
            isolate_naked_doubles_from_masks(&candidate_set);

        for (i, j, mask) in pair_mask {
            for row in 0..9 {
                if row == i || row == j {
                    continue;
                }

                if let Cell::Candidates(ref mut bits) = board[row][col] {
                    for val_idx in 0..9 {
                        if mask[val_idx] && bits[val_idx] {
                            bits[val_idx] = false;
                            change = true;
                        }
                    }
                }
            }   
        }
    }
    
    // For box
    for r in [0,3,6] {
        for c in [0,3,6] {
            let candidate_set = 
                get_unit_candidate_masks(board, r, c, UnitMode::Box);
            let results = 
                isolate_naked_doubles_from_masks(&candidate_set);
            for (i, j, mask) in results {
                for idx in 0..9 {
                    if idx == i || idx == j {
                        continue
                    }
                    let row = r + idx / 3;
                    let col = c + idx % 3;
                    if let Cell::Candidates(ref mut bits) = board[row][col] {
                        for val_idx in 0..9 {
                            if mask[val_idx] && bits[val_idx] {
                                bits[val_idx] = false;
                                change = true;
                            }
                        }
                    }
                }
            }
        }
    }
    change
}

fn isolate_hidden_doubles_from_masks(candidate_set: &Vec<(usize, BitMask)>) 
->  Vec<(usize, BitMask)> {
    let mut results: Vec<(usize, BitMask)> = Vec::new();

    for (i, mask_i) in candidate_set {
        for (j, mask_j) in candidate_set {
            if i != j && mask_i != mask_j {
                let shared = BitMask::and(&mask_i, &mask_j);
                if shared.bits_set() == 2 {
                    // Need to assert that those two bits do not appear anywhere else
                    let mut it = shared.iter().enumerate().filter_map(|(i, &v)| v.then_some(i));
                    let (first, second) = (it.next().unwrap(), it.next().unwrap());
                    let mut unique = true;
                    for (k, mask_k) in candidate_set {
                        if i != k && j != k && (mask_k[first] || mask_k[second]) {
                            unique = false;
                            break;
                        }
                    }
                    if !unique {
                        continue;
                    }
                    results.push((*i, shared));
                }
            }
        }
    }

    results
}

pub fn determine_hidden_doubles(board: &mut Board) -> bool {
    let mut change = false;

    // For row
    for row in 0..9 {
        let candidate_set = 
            get_unit_candidate_masks(board, row, 0, UnitMode::Row);

        let results = isolate_hidden_doubles_from_masks(&candidate_set);
        for (col_i, shared) in results {
            if let Cell::Candidates(board_mask) = &mut board[row][col_i] {
                *board_mask = shared;
                change = true;
            };
            // if let Cell::Candidates(board_mask) = &mut board[row][col_j] {
            //     *board_mask = shared;
            //     change = true;
            // };
        }
    }

    // For col
    for col in 0..9 {
        let candidate_set = 
            get_unit_candidate_masks(board, 0, col, UnitMode::Column);

        let results = isolate_hidden_doubles_from_masks(&candidate_set);
        for (row_i, shared) in results {
            if let Cell::Candidates(board_mask) = &mut board[row_i][col] {
                *board_mask = shared;
                change = true;
            };
            // if let Cell::Candidates(board_mask) = &mut board[row_j][col] {
            //     *board_mask = shared;
            //     change = true;
            // };
        }
    }

    // For box
    for r in [0,3,6] {
        for c in [0,3,6] {
            let candidate_set = 
                get_unit_candidate_masks(board, r, c, UnitMode::Box);
            let results = 
                isolate_hidden_doubles_from_masks(&candidate_set);
            for (i, shared) in results {
                let row = r + i / 3;
                let col = c + i % 3;
                if let Cell::Candidates(board_mask) = &mut board[row][col] {
                    *board_mask = shared;
                    change = true;
                };   
            }
        }
    }

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

    fn candidate_board() -> Board {
        std::array::from_fn(|_| {
            std::array::from_fn(|_| Cell::Candidates([false; 9]))
        })
    }

    #[test]
    fn naked_pairs_isolated_correctly_in_row() {
        let mut board = candidate_board();
        board[3][1] = Cell::Candidates(mask(&[2, 5]));
        board[3][4] = Cell::Candidates(mask(&[2, 5, 7, 8]));
        board[3][7] = Cell::Candidates(mask(&[2, 5]));

        assert!(determine_naked_doubles(&mut board));
        let correct_mask = mask(&[7,8]);
        assert!(matches!(board[3][4], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn naked_pairs_isolated_correctly_single_in_row() {
        let mut board = candidate_board();
        board[3][1] = Cell::Candidates(mask(&[2, 5]));
        board[3][4] = Cell::Candidates(mask(&[2, 7, 8]));
        board[3][7] = Cell::Candidates(mask(&[2, 5]));

        assert!(determine_naked_doubles(&mut board));
        let correct_mask = mask(&[7,8]);
        assert!(matches!(board[3][4], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn naked_pairs_isolated_eliminates_multiple_in_row() {
        let mut board = candidate_board();
        board[3][1] = Cell::Candidates(mask(&[2, 5]));
        board[3][4] = Cell::Candidates(mask(&[2, 7, 8]));
        board[3][7] = Cell::Candidates(mask(&[2, 5]));
        board[3][8] = Cell::Candidates(mask(&[2, 5, 6, 9]));

        assert!(determine_naked_doubles(&mut board));
        let correct_mask_first = mask(&[7,8]);
        assert!(matches!(board[3][4], Cell::Candidates(actual) if actual == correct_mask_first));

        let correct_mask_second = mask(&[6,9]);
        assert!(matches!(board[3][8], Cell::Candidates(actual) if actual == correct_mask_second));
    }

    #[test]
    fn naked_pairs_isolated_correctly_in_column() {
        let mut board = candidate_board();
        board[1][3] = Cell::Candidates(mask(&[2, 5]));
        board[4][3] = Cell::Candidates(mask(&[2, 5, 7, 8]));
        board[7][3] = Cell::Candidates(mask(&[2, 5]));

        assert!(determine_naked_doubles(&mut board));
        let correct_mask = mask(&[7, 8]);
        assert!(matches!(board[4][3], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn naked_pairs_isolated_correctly_single_in_column() {
        let mut board = candidate_board();
        board[1][3] = Cell::Candidates(mask(&[2, 5]));
        board[4][3] = Cell::Candidates(mask(&[2, 7, 8]));
        board[7][3] = Cell::Candidates(mask(&[2, 5]));

        assert!(determine_naked_doubles(&mut board));
        let correct_mask = mask(&[7, 8]);
        assert!(matches!(board[4][3], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn naked_pairs_isolated_eliminates_multiple_in_column() {
        let mut board = candidate_board();
        board[1][3] = Cell::Candidates(mask(&[2, 5]));
        board[4][3] = Cell::Candidates(mask(&[2, 7, 8]));
        board[7][3] = Cell::Candidates(mask(&[2, 5]));
        board[8][3] = Cell::Candidates(mask(&[2, 5, 6, 9]));

        assert!(determine_naked_doubles(&mut board));
        let correct_mask_first = mask(&[7, 8]);
        assert!(matches!(board[4][3], Cell::Candidates(actual) if actual == correct_mask_first));

        let correct_mask_second = mask(&[6, 9]);
        assert!(matches!(board[8][3], Cell::Candidates(actual) if actual == correct_mask_second));
    }

    #[test]
    fn naked_pairs_isolated_correctly_in_box() {
        let mut board = candidate_board();
        board[0][0] = Cell::Candidates(mask(&[2, 5]));
        board[1][1] = Cell::Candidates(mask(&[2, 5, 7, 8]));
        board[2][2] = Cell::Candidates(mask(&[2, 5]));

        assert!(determine_naked_doubles(&mut board));
        let correct_mask = mask(&[7, 8]);
        assert!(matches!(board[1][1], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn naked_pairs_isolated_correctly_single_in_box() {
        let mut board = candidate_board();
        board[0][0] = Cell::Candidates(mask(&[2, 5]));
        board[1][1] = Cell::Candidates(mask(&[2, 7, 8]));
        board[2][2] = Cell::Candidates(mask(&[2, 5]));

        assert!(determine_naked_doubles(&mut board));
        let correct_mask = mask(&[7, 8]);
        assert!(matches!(board[1][1], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn naked_pairs_isolated_eliminates_multiple_in_box() {
        let mut board = candidate_board();
        board[0][0] = Cell::Candidates(mask(&[2, 5]));
        board[1][1] = Cell::Candidates(mask(&[2, 7, 8]));
        board[2][2] = Cell::Candidates(mask(&[2, 5]));
        board[0][2] = Cell::Candidates(mask(&[2, 5, 6, 9]));

        assert!(determine_naked_doubles(&mut board));
        let correct_mask_first = mask(&[7, 8]);
        assert!(matches!(board[1][1], Cell::Candidates(actual) if actual == correct_mask_first));

        let correct_mask_second = mask(&[6, 9]);
        assert!(matches!(board[0][2], Cell::Candidates(actual) if actual == correct_mask_second));
    }

    #[test]
    fn hidden_pairs_two_candidate_cells_row() {
        let mut board = candidate_board();
        board[0][0] = Cell::Candidates(mask(&[2, 4, 7, 9]));
        board[0][1] = Cell::Candidates(mask(&[1, 2, 7]));

        assert!(determine_hidden_doubles(&mut board));
        let correct_mask = mask(&[2, 7]);
        assert!(matches!(board[0][0], Cell::Candidates(actual) if actual == correct_mask));
        assert!(matches!(board[0][1], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn hidden_pairs_four_candidate_cells_row() {
        let mut board = candidate_board();
        board[0][0] = Cell::Candidates(mask(&[2, 4, 7, 9]));
        board[0][1] = Cell::Candidates(mask(&[1, 2, 7]));
        board[0][4] = Cell::Candidates(mask(&[3, 6, 8]));
        board[0][7] = Cell::Candidates(mask(&[5, 6, 8]));

        assert!(determine_hidden_doubles(&mut board));
        let correct_mask_i = mask(&[2, 7]);
        assert!(matches!(board[0][0], Cell::Candidates(actual) if actual == correct_mask_i));
        assert!(matches!(board[0][1], Cell::Candidates(actual) if actual == correct_mask_i));

        let correct_mask_j = mask(&[6, 8]);
        assert!(matches!(board[0][4], Cell::Candidates(actual) if actual == correct_mask_j));
        assert!(matches!(board[0][7], Cell::Candidates(actual) if actual == correct_mask_j));
    }

    #[test]
    fn shared_two_candidates_are_not_necessarily_a_hidden_pair_row() {
        let mut board = candidate_board();

        board[0][0] = Cell::Candidates(mask(&[2, 4, 7, 9]));
        board[0][1] = Cell::Candidates(mask(&[1, 2, 7]));
        board[0][2] = Cell::Candidates(mask(&[2, 3]));

        let before = board;

        assert!(!determine_hidden_doubles(&mut board));
        assert_eq!(board, before);
    }

    #[test]
    fn hidden_pairs_two_candidate_cells_column() {
        let mut board = candidate_board();
        board[0][3] = Cell::Candidates(mask(&[2, 4, 7, 9]));
        board[1][3] = Cell::Candidates(mask(&[1, 2, 7]));

        assert!(determine_hidden_doubles(&mut board));
        let correct_mask = mask(&[2, 7]);
        assert!(matches!(board[0][3], Cell::Candidates(actual) if actual == correct_mask));
        assert!(matches!(board[1][3], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn hidden_pairs_four_candidate_cells_column() {
        let mut board = candidate_board();
        board[0][3] = Cell::Candidates(mask(&[2, 4, 7, 9]));
        board[1][3] = Cell::Candidates(mask(&[1, 2, 7]));
        board[4][3] = Cell::Candidates(mask(&[3, 6, 8]));
        board[7][3] = Cell::Candidates(mask(&[5, 6, 8]));

        assert!(determine_hidden_doubles(&mut board));
        let correct_mask_first = mask(&[2, 7]);
        assert!(matches!(board[0][3], Cell::Candidates(actual) if actual == correct_mask_first));
        assert!(matches!(board[1][3], Cell::Candidates(actual) if actual == correct_mask_first));

        let correct_mask_second = mask(&[6, 8]);
        assert!(matches!(board[4][3], Cell::Candidates(actual) if actual == correct_mask_second));
        assert!(matches!(board[7][3], Cell::Candidates(actual) if actual == correct_mask_second));
    }

    #[test]
    fn shared_two_candidates_are_not_necessarily_a_hidden_pair_column() {
        let mut board = candidate_board();

        board[0][3] = Cell::Candidates(mask(&[2, 4, 7, 9]));
        board[1][3] = Cell::Candidates(mask(&[1, 2, 7]));
        board[2][3] = Cell::Candidates(mask(&[2, 3]));

        let before = board;

        assert!(!determine_hidden_doubles(&mut board));
        assert_eq!(board, before);
    }

    #[test]
    fn hidden_pairs_two_candidate_cells_box() {
        let mut board = candidate_board();
        board[0][0] = Cell::Candidates(mask(&[2, 4, 7, 9]));
        board[1][1] = Cell::Candidates(mask(&[1, 2, 7]));

        assert!(determine_hidden_doubles(&mut board));
        let correct_mask = mask(&[2, 7]);
        assert!(matches!(board[0][0], Cell::Candidates(actual) if actual == correct_mask));
        assert!(matches!(board[1][1], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn hidden_pairs_four_candidate_cells_box() {
        let mut board = candidate_board();
        board[0][0] = Cell::Candidates(mask(&[2, 4, 7, 9]));
        board[1][1] = Cell::Candidates(mask(&[1, 2, 7]));
        board[1][2] = Cell::Candidates(mask(&[3, 6, 8]));
        board[2][1] = Cell::Candidates(mask(&[5, 6, 8]));

        assert!(determine_hidden_doubles(&mut board));
        let correct_mask_first = mask(&[2, 7]);
        assert!(matches!(board[0][0], Cell::Candidates(actual) if actual == correct_mask_first));
        assert!(matches!(board[1][1], Cell::Candidates(actual) if actual == correct_mask_first));

        let correct_mask_second = mask(&[6, 8]);
        assert!(matches!(board[1][2], Cell::Candidates(actual) if actual == correct_mask_second));
        assert!(matches!(board[2][1], Cell::Candidates(actual) if actual == correct_mask_second));
    }

    #[test]
    fn shared_two_candidates_are_not_necessarily_a_hidden_pair_box() {
        let mut board = candidate_board();

        board[0][0] = Cell::Candidates(mask(&[2, 4, 7, 9]));
        board[1][1] = Cell::Candidates(mask(&[1, 2, 7]));
        board[2][2] = Cell::Candidates(mask(&[2, 3]));

        let before = board;

        assert!(!determine_hidden_doubles(&mut board));
        assert_eq!(board, before);
    }
}
