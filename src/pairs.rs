use crate::helpers::*;
use crate::types::*;
use crate::subsets::iterate_for_subsets;

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
            get_unit_candidate_masks(board, row, UnitMode::Row);
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
            get_unit_candidate_masks(board, col, UnitMode::Column);
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
    for box_idx in 0..9 {
        let candidate_set = 
            get_unit_candidate_masks(board, box_idx, UnitMode::Box);
        let results = 
            isolate_naked_doubles_from_masks(&candidate_set);
        for (i, j, mask) in results {
            for idx in 0..9 {
                if idx == i || idx == j {
                    continue
                }
                let row_start = (box_idx / 3) * 3;
                let col_start = (box_idx % 3) * 3;
                let row = row_start + idx / 3;
                let col = col_start + idx % 3;
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
    change
}

pub fn determine_hidden_doubles(board: &mut Board) -> bool {
    let mut change = false;
    let indices: Vec<usize> = (0..9).collect();

    for mode in UnitMode::LIST {
        for index in 0..9 {
            // Iterating through all 9 rows, columns, and boxes
            let mut candidate_set = 
                get_unit_candidate_masks(board, index, mode);

            // Positions[value] is every index where value is a candidate
            let mut positions: [BitMask; 9] = [[false; 9] ; 9];
            for (idx, mask) in &candidate_set {
                for value in 0..9{
                    if mask[value] {
                        positions[value][*idx] = true;
                    }
                }
            }
            let mut subset: Vec<usize> = Vec::new();
            let mut permutations: Vec<Vec<usize>> = Vec::new();
            iterate_for_subsets(&indices, 2, 0, &mut subset, &mut permutations);

            for p in permutations {
                let mut union: BitMask = [false; 9];
                let mut candidate = true;
                for idx in &p {
                    if positions[*idx].bits_set() < 2 as usize{
                        candidate = false;
                    }
                    union.or(&positions[*idx]);
                }
                if !candidate {
                    continue;
                }
                if union.bits_set() == 2 {
                    // P holds the values which were found in the indices 
                    for (idx, mask) in &mut candidate_set {
                        if union[*idx] {
                            for value in 0..9 {
                                if !p.contains(&value) {
                                    mask[value] = false;
                                }
                            }
                        }
                    }
                }
            }
            change |= set_unit_candidate_masks(board, index, &candidate_set, mode);
            
        }
    }

    change
}

#[cfg(test)]
mod tests { 
    use crate::test_puzzles::MED_FAIL;
    use crate::eliminate_candidates;

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
    fn naked_pairs_isolated_correctly_in_row() {
        let mut board = candidate_board(false);
        board[3][1] = Cell::Candidates(mask(&[2, 5]));
        board[3][4] = Cell::Candidates(mask(&[2, 5, 7, 8]));
        board[3][7] = Cell::Candidates(mask(&[2, 5]));

        assert!(determine_naked_doubles(&mut board));
        let correct_mask = mask(&[7,8]);
        assert!(matches!(board[3][4], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn naked_pairs_isolated_correctly_single_in_row() {
        let mut board = candidate_board(false);
        board[3][1] = Cell::Candidates(mask(&[2, 5]));
        board[3][4] = Cell::Candidates(mask(&[2, 7, 8]));
        board[3][7] = Cell::Candidates(mask(&[2, 5]));

        assert!(determine_naked_doubles(&mut board));
        let correct_mask = mask(&[7,8]);
        assert!(matches!(board[3][4], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn naked_pairs_isolated_eliminates_multiple_in_row() {
        let mut board = candidate_board(false);
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
        let mut board = candidate_board(false);
        board[1][3] = Cell::Candidates(mask(&[2, 5]));
        board[4][3] = Cell::Candidates(mask(&[2, 5, 7, 8]));
        board[7][3] = Cell::Candidates(mask(&[2, 5]));

        assert!(determine_naked_doubles(&mut board));
        let correct_mask = mask(&[7, 8]);
        assert!(matches!(board[4][3], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn naked_pairs_isolated_correctly_single_in_column() {
        let mut board = candidate_board(false);
        board[1][3] = Cell::Candidates(mask(&[2, 5]));
        board[4][3] = Cell::Candidates(mask(&[2, 7, 8]));
        board[7][3] = Cell::Candidates(mask(&[2, 5]));

        assert!(determine_naked_doubles(&mut board));
        let correct_mask = mask(&[7, 8]);
        assert!(matches!(board[4][3], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn naked_pairs_isolated_eliminates_multiple_in_column() {
        let mut board = candidate_board(false);
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
        let mut board = candidate_board(false);
        board[0][0] = Cell::Candidates(mask(&[2, 5]));
        board[1][1] = Cell::Candidates(mask(&[2, 5, 7, 8]));
        board[2][2] = Cell::Candidates(mask(&[2, 5]));

        assert!(determine_naked_doubles(&mut board));
        let correct_mask = mask(&[7, 8]);
        assert!(matches!(board[1][1], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn naked_pairs_isolated_correctly_single_in_box() {
        let mut board = candidate_board(false);
        board[0][0] = Cell::Candidates(mask(&[2, 5]));
        board[1][1] = Cell::Candidates(mask(&[2, 7, 8]));
        board[2][2] = Cell::Candidates(mask(&[2, 5]));

        assert!(determine_naked_doubles(&mut board));
        let correct_mask = mask(&[7, 8]);
        assert!(matches!(board[1][1], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn naked_pairs_isolated_eliminates_multiple_in_box() {
        let mut board = candidate_board(false);
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
        let mut board = candidate_board(true);
        board[0][0] = Cell::Candidates(mask(&[2, 4, 7, 9]));
        board[0][1] = Cell::Candidates(mask(&[1, 2, 7]));
        for col in 2..9 {
            if let Cell::Candidates(mask) = &mut board[0][col] {
                mask[1] = false;
                mask[6] = false;
            }
        }

        assert!(determine_hidden_doubles(&mut board));
        let correct_mask = Cell::Candidates(mask(&[2, 7]));
        assert_eq!(board[0][0], correct_mask);
        assert_eq!(board[0][1], correct_mask);
    }

    #[test]
    fn hidden_pairs_four_candidate_cells_row() {
        let mut board = candidate_board(false);
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
        let mut board = candidate_board(true);

        board[0][0] = Cell::Candidates(mask(&[2, 4, 7, 9]));
        board[0][1] = Cell::Candidates(mask(&[1, 2, 7]));
        board[0][2] = Cell::Candidates(mask(&[2, 3]));

        let before = board;

        assert!(!determine_hidden_doubles(&mut board));
        assert_eq!(board, before);
    }

    #[test]
    fn hidden_pairs_two_candidate_cells_column() {
        let mut board = candidate_board(false);
        board[0][3] = Cell::Candidates(mask(&[2, 4, 7, 9]));
        board[1][3] = Cell::Candidates(mask(&[1, 2, 7]));

        assert!(determine_hidden_doubles(&mut board));
        let correct_mask = mask(&[2, 7]);
        assert!(matches!(board[0][3], Cell::Candidates(actual) if actual == correct_mask));
        assert!(matches!(board[1][3], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn hidden_pairs_four_candidate_cells_column() {
        let mut board = candidate_board(false);
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
        let mut board = candidate_board(true);

        board[0][3] = Cell::Candidates(mask(&[2, 4, 7, 9]));
        board[1][3] = Cell::Candidates(mask(&[1, 2, 7]));
        board[2][3] = Cell::Candidates(mask(&[2, 3]));

        let before = board;

        assert!(!determine_hidden_doubles(&mut board));
        assert_eq!(board, before);
    }

    #[test]
    fn hidden_pairs_two_candidate_cells_box() {
        let mut board = candidate_board(false);
        board[0][0] = Cell::Candidates(mask(&[2, 4, 7, 9]));
        board[1][1] = Cell::Candidates(mask(&[1, 2, 7]));

        assert!(determine_hidden_doubles(&mut board));
        let correct_mask = mask(&[2, 7]);
        assert!(matches!(board[0][0], Cell::Candidates(actual) if actual == correct_mask));
        assert!(matches!(board[1][1], Cell::Candidates(actual) if actual == correct_mask));
    }

    #[test]
    fn hidden_pairs_four_candidate_cells_box() {
        let mut board = candidate_board(false);
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
        let mut board = candidate_board(true);

        board[0][0] = Cell::Candidates(mask(&[2, 4, 7, 9]));
        board[1][1] = Cell::Candidates(mask(&[1, 2, 7]));
        board[2][2] = Cell::Candidates(mask(&[2, 3]));

        let before = board;

        assert!(!determine_hidden_doubles(&mut board));
        assert_eq!(board, before);
    }

    #[test]
    fn hidden_pairs_identifies_pair_in_test_board() {
        let mut board = grid_to_cells(&MED_FAIL);
        assert!(eliminate_candidates(&mut board));
        for row in [3, 4, 5] {
            for col in [3, 4, 5] {
                if col == 4 {
                    if row == 3 || row == 4 {
                        continue;
                    }
                }
                if let Cell::Candidates(bits) = board[row][col] {
                    assert!(!bits[2] && !bits[3]);
                };
                // Asserted that 3 and 4 are only possible at (3,4) and (4,4)
            }
        }

        assert!(determine_hidden_doubles(&mut board));

        let correct_mask = Cell::Candidates(mask(&[3,4]));
        assert_eq!(board[3][4], correct_mask);
        assert_eq!(board[4][4], correct_mask);
    }
}
