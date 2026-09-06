use crate::helpers::*;
use crate::types::*;

// fn factorial(n: usize) -> usize {
//     if n == 0 { return 1 ;}
//     (1..=n).fold(1, |acc, x| acc * x)
// }

// fn increment(list: &mut [bool], size: usize) -> bool {
//     let len = list.len();
//     let count = list.iter().filter(|x| **x == true).count();
//     // Last index of a true value
//     let mut pos = list.iter().rev().position(|x| *x == true).unwrap();

//     // Base case
//     if count == 1 && size == 1 && pos == len - 1 {
//         return false; 
//     }

//     if count == size && pos < len - 1 {
//         list[pos] = false;
//         list[pos + 1] = true;
//         return true;
//     }

//     if count == size {
//         if !increment(&mut list[0..len-1], size - 1) {
//             return false
//         }
//         list[pos] = false;
//         pos = list.iter().rev().position(|x| *x == true).unwrap();
//         list[pos + 1] = true;
//     }
    
//     if count < size {
        
//         list[pos] = true;
//         return true;
//     }
//     false
// }

fn iterate_for_subsets(set: &Vec<(usize, BitMask)>, 
                        size: usize, 
                        start: usize,
                        subset: &mut Vec<(usize, BitMask)>,
                        result: &mut Vec<Vec<(usize, BitMask)>>) {
    let len = set.len();
    let sublen = subset.len();
    
    if sublen == size {
        result.push(subset.clone());
        // subset.clear();
        return
    }

    for i in start..len {
        if (len - i) < (size - sublen){
            break;
        }
        subset.push(set[i]);
        iterate_for_subsets(set, size, i + 1, subset, result);
        subset.pop();
    }
}

fn generate_naked_subsets(candidate_set: &Vec<(usize, BitMask)>, size: usize) -> Vec<Vec<(usize, BitMask)>> {
    let mut result: Vec<Vec<(usize, BitMask)>> = Vec::new();
    let len = candidate_set.len();
    if len < size {
        return result;
    }
    if len == size {
        result.push(candidate_set.clone());
        return result;
    }
    let mut sub_result: Vec<(usize, BitMask)> = Vec::new();
    
    iterate_for_subsets(candidate_set, size, 0, &mut sub_result, &mut result);
    result
}

// A generalization of naked pairs, where k cells in a unit 
// collectively contain only k possible values (here k goes up to 4)
// So the rest of the cells eliminate those k values
pub fn determine_naked_subsets(board: &mut Board) -> bool {
    let mut change = false;

    for row in 0..9 {
        let candidate_set = 
            get_unit_candidate_masks(board, row, 0, UnitMode::Row);
        
        for k in [2, 3, 4] {
            let combinations = generate_naked_subsets(&candidate_set, k);
            if combinations.len() == 0 {
                continue;
            }
            let mut indices: Vec<usize> = Vec::new();
            for combo in combinations {
                debug_assert!(combo.len() == k);
                let mut result: BitMask = [false; 9];
                for (index, mask) in combo {
                    indices.push(index);
                    result.or(&mask);
                }
                if result.bits_set() == k {
                    for col in 0..9 {
                        if indices.contains(&col) {
                            continue;
                        }
                        if let Cell::Candidates(ref mut mask) = board[row][col] {
                            if mask.remove(&result) {
                                change = true;
                                debug_assert!(mask.bits_set() > 0);
                            }
                        }
                    }
                }
                indices.clear();
            }
        }
    }
    
    for col in 0..9 {
        let candidate_set = 
            get_unit_candidate_masks(board, 0, col, UnitMode::Column);
        
        for k in [2, 3, 4] {
            let combinations = generate_naked_subsets(&candidate_set, k);
            if combinations.len() == 0 {
                continue;
            }
            let mut indices: Vec<usize> = Vec::new();
            for combo in combinations {
                debug_assert!(combo.len() == k);
                let mut result: BitMask = [false; 9];
                for (index, mask) in combo {
                    indices.push(index);
                    result.or(&mask);
                }
                if result.bits_set() == k {
                    for row in 0..9 {
                        if indices.contains(&row) {
                            continue;
                        }
                        if let Cell::Candidates(ref mut mask) = board[row][col] {
                            if mask.remove(&result) {
                                change = true;
                                debug_assert!(mask.bits_set() > 0);
                            }
                        }
                    }
                }
                indices.clear();
            }
        }
    }

    for r in [0, 3, 6] {
        for c in [0, 3, 6] {
            let candidate_set = 
                get_unit_candidate_masks(board, r, c, UnitMode::Box);

            for k in [2, 3, 4] {
                let combinations = generate_naked_subsets(&candidate_set, k);
                if combinations.len() == 0 {
                    continue;
                }
                let mut indices: Vec<usize> = Vec::new();
                for combo in combinations {
                    debug_assert!(combo.len() == k);
                    let mut result: BitMask = [false; 9];
                    for (index, mask) in combo {
                        // We don't want to change the indices of the combo in consideration
                        indices.push(index);
                        result.or(&mask);
                    }
                    if result.bits_set() == k {
                        for row_idx in 0..3 {
                            for col_idx in 0..3 {
                                let idx = row_idx * 3 + col_idx;
                                if indices.contains(&idx) {
                                    continue;
                                }
                                let row = r + row_idx;
                                let col = c + col_idx;
                                if let Cell::Candidates(ref mut mask) = board[row][col] {
                                    if mask.remove(&result) {
                                        change = true;
                                        debug_assert!(mask.bits_set() > 0);
                                    }
                                }
                            }
                        }
                    }
                    indices.clear();
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
    fn determine_naked_subsets_isolates_triple_in_row() {
        let mut board = candidate_board(true);
        board[3][1] = Cell::Candidates(mask(&[1, 2]));
        board[3][4] = Cell::Candidates(mask(&[2, 3]));
        board[3][7] = Cell::Candidates(mask(&[1, 3]));
        board[3][8] = Cell::Candidates(mask(&[1, 2, 3, 6, 9]));

        assert!(determine_naked_subsets(&mut board));
        let mut correct_mask = mask(&[6, 9]);
        assert!(matches!(board[3][8], Cell::Candidates(actual) if actual == correct_mask));
        correct_mask = mask( &[4, 5, 6, 7, 8, 9]);
        for col in 0..9 {
            if [1,4,7,8].contains(&col) {
                continue;
            }
            if let Cell::Candidates(mask) = board[3][col] {
                assert_eq!(mask, correct_mask);
            }
        }
    }

    #[test]
    fn determine_naked_subsets_ignores_non_triple_in_row() {
        let mut board = candidate_board(true);
        board[3][1] = Cell::Candidates(mask(&[1, 2]));
        board[3][4] = Cell::Candidates(mask(&[2, 3]));
        board[3][7] = Cell::Candidates(mask(&[1, 3, 4]));
        board[3][8] = Cell::Candidates(mask(&[1, 2, 3, 6, 9]));

        assert!(!determine_naked_subsets(&mut board));
        for col in 0..9 {
            if [1,4,7,8].contains(&col) {
                continue;
            }
            if let Cell::Candidates(mask) = board[3][col] {
                assert!(mask.bits_set() == 9);
            }
        }
    }

    #[test]
    fn determine_naked_subsets_isolates_triple_in_column() {
        let mut board = candidate_board(true);
        board[1][3] = Cell::Candidates(mask(&[1, 2]));
        board[4][3] = Cell::Candidates(mask(&[2, 3]));
        board[7][3] = Cell::Candidates(mask(&[1, 3]));
        board[8][3] = Cell::Candidates(mask(&[1, 2, 3, 6, 9]));

        assert!(determine_naked_subsets(&mut board));
        let mut correct_mask = mask(&[6, 9]);
        assert!(matches!(board[8][3], Cell::Candidates(actual) if actual == correct_mask));
        correct_mask = mask( &[4, 5, 6, 7, 8, 9]);
        for row in 0..9 {
            if [1,4,7,8].contains(&row) {
                continue;
            }
            if let Cell::Candidates(mask) = board[row][3] {
                assert_eq!(mask, correct_mask);
            }
        }
    }

    #[test]
    fn determine_naked_subsets_ignores_non_triple_in_column() {
        let mut board = candidate_board(true);
        board[1][3] = Cell::Candidates(mask(&[1, 2]));
        board[4][3] = Cell::Candidates(mask(&[2, 3]));
        board[7][3] = Cell::Candidates(mask(&[1, 3, 4]));
        board[8][3] = Cell::Candidates(mask(&[1, 2, 3, 6, 9]));

        assert!(!determine_naked_subsets(&mut board));
        for row in 0..9 {
            if [1,4,7,8].contains(&row) {
                continue;
            }
            if let Cell::Candidates(mask) = board[row][3] {
                assert!(mask.bits_set() == 9);
            }
        }
    }

    #[test]
    fn determine_naked_subsets_isolates_triple_in_box() {
        let mut board = candidate_board(true);
        board[0][0] = Cell::Candidates(mask(&[1, 2]));
        board[0][1] = Cell::Candidates(mask(&[2, 3]));
        board[1][1] = Cell::Candidates(mask(&[1, 3]));
        board[2][2] = Cell::Candidates(mask(&[1, 2, 3, 6, 9]));

        assert!(determine_naked_subsets(&mut board));
        let mut correct_mask = mask(&[6, 9]);
        assert!(matches!(board[2][2], Cell::Candidates(actual) if actual == correct_mask));
        correct_mask = mask( &[4, 5, 6, 7, 8, 9]);
        for row in 0..3 {
            for col in 0..3 {
                if [(0, 0), (0, 1), (1, 1), (2, 2)].contains(&(row, col))  {
                    continue;
                }
                if let Cell::Candidates(mask) = board[row][col] {
                    assert_eq!(mask, correct_mask, "Assertion failed at ({row}, {col})");
                }
            }
        }
    }

    #[test]
    fn determine_naked_subsets_ignores_non_triple_in_box() {
        let mut board = candidate_board(true);
        board[0][0] = Cell::Candidates(mask(&[1, 2]));
        board[0][1] = Cell::Candidates(mask(&[2, 3]));
        board[1][1] = Cell::Candidates(mask(&[1, 3, 4]));
        board[2][2] = Cell::Candidates(mask(&[1, 2, 3, 6, 9]));

        assert!(!determine_naked_subsets(&mut board));
        for row in 0..3 {
            for col in 0..3 {
                if [(0, 0), (0, 1), (1, 1), (2, 2)].contains(&(row, col))  {
                    continue;
                }
                if let Cell::Candidates(mask) = board[row][col] {
                    assert!(mask.bits_set() == 9);
                }
            }
        }
    }

    // #[test]
    // fn increment_increments_normally() {
    //     let mut test_case = vec![true, true, true, false, false];
    //     assert!(increment(&mut test_case, 3));
    //     assert_eq!(test_case, vec![true, true, false, true, false]);
    // }

    // #[test]
    // fn increment_increments_initially() {
    //     let mut test_case = vec![true, true, false, false, false];
    //     assert!(increment(&mut test_case, 3));
    //     assert_eq!(test_case, vec![true, true, true, false, false]);
    // }

    // #[test]
    // fn increment_wraps_around() {
    //     let mut test_case = vec![true, true, false, false, true];
    //     assert!(increment(&mut test_case, 3));
    //     assert_eq!(test_case, vec![true, false, true, true, false]);
    // }

    // #[test]
    // fn increment_increments_near_end() {
    //     let mut test_case = vec![true, false, false, true, true];
    //     assert!(increment(&mut test_case, 3));
    //     assert_eq!(test_case, vec![false, true, false, true, true]);
    // }

    // #[test]
    // fn increment_starts_cleanly() {
    //     let mut test_case = vec![false, false, false, false, false];
    //     assert!(increment(&mut test_case, 3));
    //     assert_eq!(test_case, vec![true, false, false, false, false]);
    // }

    // #[test]
    // fn increment_finishes_cleanly() {
    //     let mut test_case = vec![false, false, true, true, true];
    //     assert!(!increment(&mut test_case, 3));
    // }
}