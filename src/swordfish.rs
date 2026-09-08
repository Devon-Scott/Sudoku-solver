use crate::helpers::*;
use crate::types::*;
use crate::subsets::iterate_for_subsets;

fn generate_union_intersect(masks: &Vec<Vec<(usize, [bool; 9])>>, value: usize) -> (BitMask, BitMask) {
    let mut union: BitMask = [false; 9];
    let mut intersect: BitMask = [true; 9];
    for mask_set in masks {
        let mut seen: BitMask = [false; 9];
        for (index, mask) in mask_set {
            if mask[value] {
                seen[*index] = true;
            }
        }
        union.or(&seen);
        intersect.and(&seen);
    }
    (union, intersect)
}

// Find k rows/cols where the candidate-column sets 
// for a digit are exclusively the same k columns/rows.
//        c3        c8
//     r2 7? ------ 7?
//        |         |
//        |         |
//     r6 7? ------ 7?
// Then eliminate that digit from the rest of the candidates
// in those columns/rows
pub fn swordfish_elimination(board: &mut Board) -> bool {
    let mut change = false;

    let indices: Vec<usize> = (0..9).collect();

    for k in [2, 3, 4] {
        // Rows 
        // Generate all permutations of k rows
        let mut subset: Vec<usize> = Vec::new();
        let mut permutations: Vec<Vec<usize>> = Vec::new();
        iterate_for_subsets(&indices, k, 0, &mut subset, &mut permutations);

        for p in &permutations {
            let mut masks = Vec::new();
            for row in p {
                masks.push(get_unit_candidate_masks(board, *row, UnitMode::Row));
            }            
            
            for value in 0..9 {
                // The indices across the k rows that all share value as a possibility
                let (union, intersect) = generate_union_intersect(&masks, value);
                
                if union == intersect && union.bits_set() == k {
                    let cols: Vec<usize> = union.iter()
                        .enumerate()
                        .filter_map(|(index, &value)| value.then_some(index))
                        .collect();
                    let rows: Vec<usize> = (0..9).filter(|x| !p.contains(x)).collect();
                    for col in cols {
                        for row in &rows {
                            if let Cell::Candidates(mask) = &mut board[*row][col] && mask[value] {
                                mask[value] = false;
                                change = true;
                            }
                        }
                    }
                }
            } 
            
        }

        // Columns
        for p in &permutations {
            let mut masks = Vec::new();
            for col in p {
                masks.push(get_unit_candidate_masks(board, *col, UnitMode::Column));
            }

            for value in 0..9 {
                let (union, intersect) = generate_union_intersect(&masks, value);

                if union == intersect && union.bits_set() == k {
                    let rows: Vec<usize> = union.iter()
                        .enumerate()
                        .filter_map(|(index, &value)| value.then_some(index))
                        .collect();
                    let cols: Vec<usize> = (0..9).filter(|x| !p.contains(x)).collect();
                    for row in rows {
                        for col in &cols {
                            if let Cell::Candidates(mask) = &mut board[row][*col] && mask[value] {
                                mask[value] = false;
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
    fn swordfish_eliminates_from_pair_in_row() {
        let mut board = candidate_board(true);
        for row in [2, 6] {
            for col in 0..9 {
                if col == 3 || col == 8 {
                    continue;
                }
                if let Cell::Candidates(m) = &mut board[row][col]{
                    m[6] = false;
                }
            }
        }
        assert!(swordfish_elimination(&mut board));
        for row in 0..9 {
            for col in [3, 8] {
                if row == 2 || row == 6 {
                    if let Cell::Candidates(m) = &board[row][col]{
                        assert!(m[6], "Expected value of 7 to be true at ({row}, {col})");
                    }
                }
                else if let Cell::Candidates(m) = &board[row][col]{
                    assert!(!m[6], "Expected value of 7 to be false at ({row}, {col})");
                }
            }
        }
    }

    #[test]
    fn swordfish_ignores_extra_in_pair_in_row() {
        let mut board = candidate_board(true);
        // Two rows, but three columns, where 7 is a candidate
        for row in [2, 6] {
            for col in 0..9 {
                if col == 3 || col == 5 || col == 8 {
                    continue;
                }
                if let Cell::Candidates(m) = &mut board[row][col]{
                    m[6] = false;
                }
            }
        }
        assert!(!swordfish_elimination(&mut board));
    }

    #[test]
    fn swordfish_eliminates_from_triple_in_row() {
        let mut board = candidate_board(true);
        for row in [2, 5, 6] {
            for col in 0..9 {
                if col == 3 || col == 4 || col == 8 {
                    continue;
                }
                if let Cell::Candidates(m) = &mut board[row][col]{
                    m[6] = false;
                }
            }
        }
        assert!(swordfish_elimination(&mut board));
        for row in 0..9 {
            for col in [3, 4, 8] {
                if row == 2 || row == 5 || row == 6 {
                    if let Cell::Candidates(m) = &board[row][col]{
                        assert!(m[6], "Expected value of 7 to be true at ({row}, {col})");
                    }
                }
                else if let Cell::Candidates(m) = &board[row][col]{
                    assert!(!m[6], "Expected value of 7 to be false at ({row}, {col})");
                }
            }
        }
    }

    #[test]
    fn swordfish_ignores_extra_in_triple_in_row() {
        let mut board = candidate_board(true);
        // Three rows, but four columns, where 7 is a candidate
        for row in [2, 5, 6] {
            for col in 0..9 {
                if col == 3 || col == 4 || col == 5 || col == 8 {
                    continue;
                }
                if let Cell::Candidates(m) = &mut board[row][col]{
                    m[6] = false;
                }
            }
        }
        assert!(!swordfish_elimination(&mut board));
    }



    #[test]
    fn swordfish_eliminates_from_pair_in_column() {
        let mut board = candidate_board(true);
        for col in [2, 6] {
            for row in 0..9 {
                if row == 3 || row == 8 {
                    continue;
                }
                if let Cell::Candidates(m) = &mut board[row][col]{
                    m[6] = false;
                }
            }
        }
        assert!(swordfish_elimination(&mut board));
        for col in 0..9 {
            for row in [3, 8] {
                if col == 2 || col == 6 {
                    if let Cell::Candidates(m) = &board[row][col]{
                        assert!(m[6], "Expected value of 7 to be true at ({row}, {col})");
                    }
                }
                else if let Cell::Candidates(m) = &board[row][col]{
                    assert!(!m[6], "Expected value of 7 to be false at ({row}, {col})");
                }
            }
        }
    }

    #[test]
    fn swordfish_ignores_extra_in_pair_in_column() {
        let mut board = candidate_board(true);
        // Two columns, but three rows, where 7 is a candidate
        for col in [2, 6] {
            for row in 0..9 {
                if row == 3 || row == 5 || row == 8 {
                    continue;
                }
                if let Cell::Candidates(m) = &mut board[row][col]{
                    m[6] = false;
                }
            }
        }
        assert!(!swordfish_elimination(&mut board));
    }

    #[test]
    fn swordfish_eliminates_from_triple_in_column() {
        let mut board = candidate_board(true);
        for col in [2, 5, 6] {
            for row in 0..9 {
                if row == 3 || row == 4 || row == 8 {
                    continue;
                }
                if let Cell::Candidates(m) = &mut board[row][col]{
                    m[6] = false;
                }
            }
        }
        assert!(swordfish_elimination(&mut board));
        for col in 0..9 {
            for row in [3, 4, 8] {
                if col == 2 || col == 5 || col == 6 {
                    if let Cell::Candidates(m) = &board[row][col]{
                        assert!(m[6], "Expected value of 7 to be true at ({row}, {col})");
                    }
                }
                else if let Cell::Candidates(m) = &board[row][col]{
                    assert!(!m[6], "Expected value of 7 to be false at ({row}, {col})");
                }
            }
        }
    }

    #[test]
    fn swordfish_ignores_extra_in_triple_in_column() {
        let mut board = candidate_board(true);
        // Three columns, but four rows, where 7 is a candidate
        for col in [2, 5, 6] {
            for row in 0..9 {
                if row == 3 || row == 4 || row == 5 || row == 8 {
                    continue;
                }
                if let Cell::Candidates(m) = &mut board[row][col]{
                    m[6] = false;
                }
            }
        }
        assert!(!swordfish_elimination(&mut board));
    }
}