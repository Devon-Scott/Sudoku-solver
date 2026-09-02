use crate::types::*;

pub fn eliminate_candidates(board: &mut Board) -> bool {
    let mut change = false;
    for row in 0..9 {
        for col in 0..9 {
            if let Cell::Value(value) = board[row][col]{
                let num: usize = (value - 1) as usize;
                // Set all candidate values for this num, in this row, col, box, to false
                for c in 0..9 {
                    if let Cell::Candidates(ref mut set) = board[row][c] {
                        if set[num] {
                            set[num] = false;
                            change = true;
                        }
                    }
                }
                for r in 0..9 {
                    if let Cell::Candidates(ref mut set) = board[r][col] {
                        if set[num] {
                            set[num] = false;
                            change = true;
                        }
                    }
                }

                let box_row = row / 3;
                let box_col = col / 3;

                for row_idx in 0..3 {
                    for col_idx in 0..3 {
                        let r: usize = box_row * 3 + row_idx;
                        let c: usize = box_col * 3 + col_idx;
                        if let Cell::Candidates(ref mut set) = board[r][c] {
                            if set[num] {
                                set[num] = false;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate_board() -> Board {
        std::array::from_fn(|_| {
            std::array::from_fn(|_| Cell::Candidates([true; 9]))
        })
    }

    #[test]
    fn eliminates_value_from_its_row_column_and_box() {
        let mut board = candidate_board();
        board[1][1] = Cell::Value(6);

        assert!(eliminate_candidates(&mut board));

        for col in 0..9 {
            if let Cell::Candidates(mask) = board[1][col] {
                assert!(!mask[5], "6 remained in row 1, column {col}");
            }
        }
        for row in 0..9 {
            if let Cell::Candidates(mask) = board[row][1] {
                assert!(!mask[5], "6 remained in row {row}, column 1");
            }
        }
        for row in 0..3 {
            for col in 0..3 {
                if let Cell::Candidates(mask) = board[row][col] {
                    assert!(!mask[5], "6 remained in box cell ({row}, {col})");
                }
            }
        }
    }

    #[test]
    fn leaves_unrelated_candidates_unchanged() {
        let mut board = candidate_board();
        board[1][1] = Cell::Value(6);

        eliminate_candidates(&mut board);

        let Cell::Candidates(mask) = board[8][8] else {
            panic!("unrelated cell unexpectedly stopped being a candidate cell");
        };
        assert_eq!(mask, [true; 9]);
    }

    #[test]
    fn reports_no_change_when_value_is_already_eliminated() {
        let mut board = candidate_board();
        board[1][1] = Cell::Value(6);
        eliminate_candidates(&mut board);

        assert!(!eliminate_candidates(&mut board));
    }
}
