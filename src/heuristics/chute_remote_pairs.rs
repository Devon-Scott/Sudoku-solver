use crate::helpers::*;
use crate::types::*;

pub fn chute_remote_pairs(board: &mut Board) -> bool {
    let mut change = false;

    // for mode in (UnitMode::Row, UnitMode::Column) {

    // }
    for row_1 in 0..9 {
        for row_2 in 0..9 {
            if row_1 == row_2 || (row_1 / 3) != (row_2 / 3) {
                // Ensure both rows are in the same row of boxes
                continue;
            }

            let r1_candidate_set = 
                get_unit_candidate_masks(board, row_1, UnitMode::Row);
            let r2_candidate_set = 
                get_unit_candidate_masks(board, row_2, UnitMode::Row);

            for (c1, rc1) in &r1_candidate_set {
                for (c2, rc2) in &r2_candidate_set {
                    if (c1 / 3) == (c2 / 3) {
                        // ensure two candidates are not in the same box
                        continue;
                    }

                    if rc1.bits_set() == 2 && rc1 == rc2 {
                        // Find the third row and third box among the three
                        let third_box = 3 - (c1 / 3) - (c2/ 3);
                        assert!(third_box != (c1 / 3));
                        assert!(third_box != (c2 / 3));
                        let base = ((row_1 / 3) * 3) * 3 + 3;
                        assert!(base >= row_1 + row_2, "Values were: base {base}, row_1 {row_1}, row_2 {row_2}");
                        let third_row = base - row_1 - row_2;

                        // TODO: I did this backwards. I need to use absence, not presence, as evidence
                        // Load the rest of the examples as test cases
                        let mut values: BitMask = [false; 9];
                        for chute in 0..3 {
                            let col = (third_box * 3) + chute;
                            if let Cell::Candidates(mask) = board[third_row][col] {
                                values.or(&mask);
                            }
                            else if let Cell::Value(value) = board[third_row][col] {
                                values[value - 1] = true;
                            }
                        }
                        if values.bits_and(rc1).bits_set() == 1 {
                            // Get the value
                            let mask_val = values.bits_and(&rc1).iter().position(|&x| x).unwrap();
                            for col in 0..9 {
                                // Cells in row_1
                                let mut visible = true;
                                visible &= visible_to(row_1, col, row_1, *c1);
                                visible &= visible_to(row_1, col, row_2, *c2);

                                if visible {
                                    if let Cell::Candidates(ref mut mask) = board[row_1][col] {
                                        if mask[mask_val] {
                                            mask[mask_val] = false;
                                            change = true;
                                            assert!(mask.bits_set() > 0);
                                        }
                                    }
                                }

                                // Cells in row 2
                                visible = true;
                                visible &= visible_to(row_2, col, row_1, *c1);
                                visible &= visible_to(row_2, col, row_2, *c2);

                                if visible {
                                    if let Cell::Candidates(ref mut mask) = board[row_2][col] {
                                        if mask[mask_val] {
                                            mask[mask_val] = false;
                                            change = true;
                                            assert!(mask.bits_set() > 0);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }


    for col_1 in 0..9 {
        for col_2 in 0..9 {
            if col_1 == col_2 || (col_1 / 3) != (col_2 / 3) {
                // Ensure both cols are in the same col of boxes
                continue;
            }

            let c1_candidate_set = 
                get_unit_candidate_masks(board, col_1, UnitMode::Column);
            let c2_candidate_set = 
                get_unit_candidate_masks(board, col_2, UnitMode::Column);

            for (r1, cc1) in &c1_candidate_set {
                for (r2, cc2) in &c2_candidate_set {
                    if (r1 / 3) == (r2 / 3) {
                        // ensure two candidates are not in the same box
                        continue;
                    }

                    if cc1.bits_set() == 2 && cc1 == cc2 {
                        // Find the third row and third box among the three
                        let third_box = 3 - (r1 / 3) - (r2/ 3);
                        let base = ((col_1 / 3) * 3) * 3 + 3;
                        assert!(base >= col_1 + col_2, "Values were: base {base}, col_1 {col_1}, col_2 {col_2}");
                        let third_col = base - col_1 - col_2;

                        let mut values: BitMask = [false; 9];
                        for chute in 0..3 {
                            let row = (third_box * 3) + chute;
                            if let Cell::Candidates(mask) = board[row][third_col] {
                                values.or(&mask);
                            }
                            else if let Cell::Value(value) = board[row][third_col] {
                                values[value - 1] = true;
                            }
                        }
                        if values.bits_and(cc1).bits_set() == 1 {
                            // Get the value
                            let mask_val = values.bits_and(&cc1).iter().position(|&x| x).unwrap();
                            for row in 0..9 {
                                // Cells in col_1
                                let mut visible = true;
                                visible &= visible_to(row, col_1, *r1, col_1);
                                visible &= visible_to(row, col_1, *r2, col_2);

                                if visible {
                                    if let Cell::Candidates(ref mut mask) = board[row][col_1] {
                                        if mask[mask_val] {
                                            mask[mask_val] = false;
                                            change = true;
                                            assert!(mask.bits_set() > 0);
                                        }
                                    }
                                }

                                // Cells in row 2
                                visible = true;
                                visible &= visible_to(row, col_2, *r1, col_1);
                                visible &= visible_to(row, col_2, *r2, col_2);

                                if visible {
                                    if let Cell::Candidates(ref mut mask) = board[row][col_2] {
                                        if mask[mask_val] {
                                            mask[mask_val] = false;
                                            change = true;
                                            assert!(mask.bits_set() > 0);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
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