use crate::helpers::*;
use crate::types::*;

// 0 1 2    A line is any exclusively horizontal
// 3 4 5    or vertical arrangement of these indices
// 6 7 8    e.g. [1, 7] is a vertical column at index 1
fn detect_lines_in_pointing_set(indices: &[usize]) -> Option<(usize, UnitMode)> {
    if indices.len() > 3 || indices.len() < 2 {
        return None
    }
    
    let first = indices[0];

    if indices.iter().all(|&index| index / 3 == first / 3) {
        return Some((first, UnitMode::Row));
    }

    if indices.iter().all(|&index| index % 3 == first % 3) {
        return Some((first, UnitMode::Column));
    }
    
    None
}

pub fn eliminate_pointing_sets(board: &mut Board) -> bool {
    let mut change = false;
    for r in [0, 3, 6] {
        for c in [0, 3, 6] {
            let candidate_set = 
                get_unit_candidate_masks(board, r, c, UnitMode::Box);
            
            // Collect all indices where value is a candidate. Return 10 from the map since it needs to be of type usize
            for value in 0..9 {
                let indices: Vec<usize> = candidate_set
                    .iter()
                    .map(|&(n, b)| { if b[value] {n} else {10} } )
                    .filter(|&n| n < 10)
                    .collect();
                if let Some((index, mode)) = detect_lines_in_pointing_set(&indices) {
                    match mode {
                        // let row = r + i / 3;
                        // let col = c + i % 3;
                        UnitMode::Row => {
                            let row = r + index / 3;
                            for col in 0..9 {
                                // skip the columns of the current box
                                if [c, c+1, c+2].contains(&col) {
                                    continue;
                                }
                                if let Cell::Candidates(board_mask) = &mut board[row][col] {
                                    if board_mask[value] {
                                        board_mask[value] = false;
                                        change = true;
                                    }
                                };
                            }
                        }
                        UnitMode::Column => {
                            let col = c + index % 3;
                            for row in 0..9 {
                                // skip the rows of the current box
                                if [r, r+1, r+2].contains(&row) {
                                    continue;
                                }
                                if let Cell::Candidates(board_mask) = &mut board[row][col] {
                                    if board_mask[value] {
                                        board_mask[value] = false;
                                        change = true;
                                    }
                                };
                            }
                        }
                        _ => {}
                    }
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
    fn eliminate_pointing_sets_eliminates_from_row() {
        let mut board = candidate_board();
        board[3][3] = Cell::Candidates(mask(&[2, 5]));
        board[3][4] = Cell::Candidates(mask(&[1, 4]));
        board[3][5] = Cell::Candidates(mask(&[3, 5]));

        board[3][0] = Cell::Candidates(mask(&[1, 5, 7]));
        board[3][6] = Cell::Candidates(mask(&[1, 5, 8]));

        assert!(eliminate_pointing_sets(&mut board));

        let correct_mask_0 = mask(&[1,7]);
        assert!(matches!(board[3][0], Cell::Candidates(actual) if actual == correct_mask_0));

        let correct_mask_1 = mask(&[1,8]);
        assert!(matches!(board[3][6], Cell::Candidates(actual) if actual == correct_mask_1));
    }

    #[test]
    fn eliminate_pointing_sets_avoids_elimination_when_not_unique_in_row() {
        let mut board = candidate_board();
        board[3][3] = Cell::Candidates(mask(&[2, 5]));
        board[4][4] = Cell::Candidates(mask(&[1, 5]));
        board[3][5] = Cell::Candidates(mask(&[3, 5]));

        board[3][0] = Cell::Candidates(mask(&[1, 5, 7]));
        board[3][6] = Cell::Candidates(mask(&[1, 5, 8]));

        assert!(!eliminate_pointing_sets(&mut board));

        let correct_mask_0 = mask(&[1, 5,7]);
        assert!(matches!(board[3][0], Cell::Candidates(actual) if actual == correct_mask_0));

        let correct_mask_1 = mask(&[1, 5, 8]);
        assert!(matches!(board[3][6], Cell::Candidates(actual) if actual == correct_mask_1));
    }

    #[test]
    fn detect_lines_in_pointing_set_identifies_row() {
        let indices = vec![0,1,2];
        let result = detect_lines_in_pointing_set(&indices);
        assert!(result == Some((0, UnitMode::Row)));
    }

    #[test]
    fn detect_lines_in_pointing_set_identifies_column() {
        let indices = vec![1,4,7];
        let result = detect_lines_in_pointing_set(&indices);
        assert!(result == Some((1, UnitMode::Column)));
    }

     #[test]
    fn detect_lines_in_pointing_set_identifies_skipped_row() {
        let indices = vec![0,2];
        let result = detect_lines_in_pointing_set(&indices);
        assert!(result == Some((0, UnitMode::Row)));
    }

    #[test]
    fn detect_lines_in_pointing_set_identifies_skipped_column() {
        let indices = vec![1,7];
        let result = detect_lines_in_pointing_set(&indices);
        assert!(result == Some((1, UnitMode::Column)));
    }

    #[test]
    fn detect_lines_in_pointing_set_rejects_row() {
        let indices = vec![0,2,3];
        let result = detect_lines_in_pointing_set(&indices);
        assert!(result == None)
    }

    #[test]
    fn detect_lines_in_pointing_set_rejects_column() {
        let indices = vec![1,5,7];
        let result = detect_lines_in_pointing_set(&indices);
        assert!(result == None)
    }
}