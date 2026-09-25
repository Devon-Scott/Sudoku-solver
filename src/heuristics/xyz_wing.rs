use crate::helpers::*;
use crate::types::*;

#[derive(PartialEq, Debug)]
struct Xyz {
    xz_row: usize,
    xz_col: usize,
    yz_row: usize,
    yz_col: usize,
    z: usize
}

// If cell 1 can "see" cell 2, return true
// That is, the two cells share a row, a column, or a box
fn visible_to(r1: usize, c1: usize, r2: usize, c2: usize) -> bool {
    // Row check
    if r1 == r2 && c1 != c2 { return true }
    if c1 == c2 && r1 != r2 { return true }

    let b1 = row_col_to_box_index(r1, c1);
    let b2 = row_col_to_box_index(r2, c2);

    if b1 == b2 && r1 != r2 && c1 != c2 { return true }

    false
}

fn xyz_detection(
    board: &Board,
    hinge: &BitMask, 
    row: usize, 
    col: usize
) -> Option<Xyz> {
    // subsets.rs should handle the case where the three cells
    // are all in the same unit. Here, XZ and YZ are theoretically
    // in different units, both visible to the hinge
    let box_index = row_col_to_box_index(row, col);

    let combinations = [
        (UnitMode::Row, UnitMode::Column),
        (UnitMode::Row, UnitMode::Box),
        (UnitMode::Column, UnitMode::Box) 
    ];

    for (xz_mode, yz_mode) in combinations {

        let xz_index = match xz_mode {
            UnitMode::Row => row,
            UnitMode::Column => col,
            UnitMode::Box => box_index
        };

        let yz_index = match yz_mode {
            UnitMode::Row => row,
            UnitMode::Column => col,
            UnitMode::Box => box_index
        };

        let xz_candidate_set =    
            get_unit_candidate_masks(board, xz_index, xz_mode);
        let yz_candidate_set = 
            get_unit_candidate_masks(board, yz_index, yz_mode);

        for (xz_idx, xz) in &xz_candidate_set {
            if xz.bits_set() == 2 && xz.bits_and(&hinge).bits_set() == 2 {
                for (yz_idx, yz) in &yz_candidate_set {
                    if yz.bits_set() == 2 && 
                        yz.bits_and(&hinge).bits_set() == 2 &&
                        yz.bits_and(&xz).bits_set() == 1 {
                            let value = yz.bits_and(&xz).iter().position(|&x| x).unwrap() + 1;

                            let xyz = match (xz_mode, yz_mode) {
                                (UnitMode::Row, UnitMode::Column) => {
                                    Xyz { 
                                        xz_row: xz_index, 
                                        xz_col: *xz_idx, 
                                        yz_row: *yz_idx, 
                                        yz_col: yz_index, 
                                        z: value
                                    }
                                }
                                (UnitMode::Row, UnitMode::Box) => {
                                    let (yz_row, yz_col) = box_index_to_row_col(yz_index, *yz_idx);
                                    Xyz { 
                                        xz_row: xz_index, 
                                        xz_col: *xz_idx, 
                                        yz_row: yz_row, 
                                        yz_col: yz_col, 
                                        z: value
                                    }
                                }
                                _ => {
                                    let (yz_row, yz_col) = box_index_to_row_col(yz_index, *yz_idx);
                                    Xyz { 
                                        xz_row: *xz_idx, 
                                        xz_col: xz_index, 
                                        yz_row: yz_row, 
                                        yz_col: yz_col, 
                                        z: value
                                    }
                                }
                            };

                            return Some(xyz);
                        }
                }
            }
        }
    }

    None
}

pub fn xyz_wing(board: &mut Board) -> bool {
    let mut change = false;

    for row in 0..9 {
        for col in 0..9 {
            if let Cell::Candidates(hinge) = board[row][col] {
                if hinge.bits_set() == 3 {
                    // Step 1: check if there are two cells visible from 
                    // the hinge that share two of the three candidate
                    // values of the hinge (XZ and YZ), where hinge has (XYZ)
                    let xyz = xyz_detection(board, &hinge, row, col);
                    if xyz.is_none() {
                        continue;
                    }

                    let result = xyz.unwrap();

                    // Step 2: Check if there are any cells visible to XZ, YZ
                    // and XYZ that also have Z as a candidate value. If so,
                    // eliminate those candidates
                    for r in 0..9 {
                        for c in 0..9 {
                            let mut visible = true;
                            visible &= visible_to(r, c, result.xz_row, result.xz_col);
                            visible &= visible_to(r, c, result.yz_row, result.yz_col);
                            visible &= visible_to(r, c, row, col);

                            if let Cell::Candidates(ref mut mask) = board[r][c] {
                                if visible && mask[result.z - 1] {
                                    mask[result.z - 1] = false;
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
    fn xyz_detection_returns_correct_candidates_row_col() {
        let mut board = candidate_board(true);
        let hinge = mask(&[1,2,4]);
        
        board[3][8] = Cell::Candidates(mask(&[1,2]));
        board[5][1] = Cell::Candidates(mask(&[1,4]));
        board[5][8] = Cell::Candidates(hinge);

        let correct_result: Xyz = Xyz { xz_row: 5, xz_col: 1, yz_row: 3, yz_col: 8, z: 1 };

        let result = xyz_detection(&board, &hinge, 5, 8);

        assert!(result.is_some());
        assert_eq!(correct_result, result.unwrap());
    }

    #[test]
    fn xyz_detection_returns_correct_candidates_row_box() {
        let mut board = candidate_board(true);
        let hinge = mask(&[4,6,7]);
        
        board[3][6] = Cell::Candidates(mask(&[6,7]));
        board[4][5] = Cell::Candidates(mask(&[4,6]));
        board[4][7] = Cell::Candidates(hinge);

        let correct_result: Xyz = Xyz { xz_row: 4, xz_col: 5, yz_row: 3, yz_col: 6, z: 6 };

        let result = xyz_detection(&board, &hinge, 4, 7);

        assert!(result.is_some());
        assert_eq!(correct_result, result.unwrap());
    }

    #[test]
    fn xyz_detection_returns_correct_candidates_col_box() {
        let mut board = candidate_board(true);
        let hinge = mask(&[4,6,7]);
        
        board[6][3] = Cell::Candidates(mask(&[6,7]));
        board[5][4] = Cell::Candidates(mask(&[4,6]));
        board[7][4] = Cell::Candidates(hinge);

        let correct_result: Xyz = Xyz { xz_row: 5, xz_col: 4, yz_row: 6, yz_col: 3, z: 6 };

        let result = xyz_detection(&board, &hinge, 7,4);

        assert!(result.is_some());
        assert_eq!(correct_result, result.unwrap());
    }

        #[test]
    fn xyz_eliminates_correct_candidates_row_col() {
        let mut board = candidate_board(true);
        let hinge = mask(&[1,2,4]);
        
        board[3][8] = Cell::Candidates(mask(&[1,2]));
        board[5][1] = Cell::Candidates(mask(&[1,4]));
        board[5][8] = Cell::Candidates(hinge);

        board[5][6] = Cell::Candidates(mask(&[1,4,8]));

        assert!(xyz_wing(&mut board));

        let correct_mask = Cell::Candidates(mask(&[4,8]));

        assert_eq!(board[5][6], correct_mask);
    }

    #[test]
    fn xyz_eliminates_correct_candidates_row_box() {
        let mut board = candidate_board(true);
        let hinge = mask(&[4,6,7]);
        
        board[3][6] = Cell::Candidates(mask(&[6,7]));
        board[4][5] = Cell::Candidates(mask(&[4,6]));
        board[4][7] = Cell::Candidates(hinge);

        board[4][8] = Cell::Candidates(mask(&[1,6,9]));

        assert!(xyz_wing(&mut board));

        let correct_mask = Cell::Candidates(mask(&[1,9]));

        assert_eq!(board[4][8], correct_mask);
    }

    #[test]
    fn xyz_eliminates_correct_candidates_col_box() {
        let mut board = candidate_board(true);
        let hinge = mask(&[4,6,7]);
        
        board[6][3] = Cell::Candidates(mask(&[6,7]));
        board[5][4] = Cell::Candidates(mask(&[4,6]));
        board[7][4] = Cell::Candidates(hinge);

        board[8][4] = Cell::Candidates(mask(&[1,6,9]));

        assert!(xyz_wing(&mut board));

        let correct_mask = Cell::Candidates(mask(&[1,9]));

        assert_eq!(board[8][4], correct_mask);
    }
}