use crate::types::*;
use crate::candidates::*;

fn get_index_of_unique_candidate(mask: &BitMask) -> Option<usize> {
    let mut idx: Option<usize> = None;
    for i in 0..9 {
        if mask[i]{
            if idx != None  {
                return None
            }
            idx = Some(i);
        }
    }
    idx
}

pub fn solve_naked_singles(board: &mut Board) -> bool {
    let mut change = false;
    for row in 0..9 {
        for col in 0..9 {
            if let Cell::Candidates(bits) = board[row][col] {
                let idx = get_index_of_unique_candidate(&bits);
                if let Some(num) = idx {
                    board[row][col] = Cell::Value((num + 1) as i16);
                    change = true;
                    eliminate_candidates(board);
                }
            }
        }
    }
    change
}

// Takes in a vector of indexes and bitmasks, and if there are hidden singles at any index,
// returns a vector of those indices, with their corresponding hidden single value
fn get_hidden_singles_from_masks(candidate_set: &Vec<(usize, BitMask)>) -> Vec<(usize, i16)> {
    let mut result: Vec<(usize, i16)> = Vec::new();
    for value in 0..9 {
        let mut seen = false;
        let mut single = true;
        let mut idx = 10;
        for mask in candidate_set {
            if mask.1[value] && !seen {
                seen = true;
                idx = mask.0;
            } 
            else if mask.1[value] && seen {
                single = false;
            }
        }
        if seen && single {
            result.push((idx, (value + 1) as i16));
        }
    }

    // If there end up being duplicate indices, don't keep any of them
    let mut count = [0; 9];
    for element in &result {
        count[element.0] += 1;
    }
    result.retain(|&pair| count[pair.0] == 1);

    result
}

pub fn solve_hidden_singles(board: &mut Board) -> bool {
    let mut change = false;
    let mut local_change = false;

    // For row
    for row in 0..9 {
        let mut candidate_set: Vec<(usize, BitMask)> = Vec::new(); 
        for col in 0..9 {
            if let Cell::Candidates(bits) = board[row][col] {
                candidate_set.push((col, bits));
            }
        }
        let results = get_hidden_singles_from_masks(&candidate_set);
        for (idx, value) in results {
            board[row][idx] = Cell::Value(value);
            change = true;
            local_change = true;
        } 
        if local_change {
            eliminate_candidates(board);
        }
        local_change = false;
    }

    // For col
    for col in 0..9 {
        let mut candidate_set: Vec<(usize, BitMask)> = Vec::new(); 
        for row in 0..9 {
            if let Cell::Candidates(bits) = board[row][col] {
                candidate_set.push((row, bits));
            }
        }
        let results = get_hidden_singles_from_masks(&candidate_set);
        for (idx, value) in results {
            board[idx][col] = Cell::Value(value);
            change = true;
            local_change = true;
        } 
        if local_change {
            eliminate_candidates(board);
        }
        local_change = false; 
    }
    
    // For box
    for r in [0,3,6] {
        for c in [0,3,6] {
            let mut candidate_set: Vec<(usize, BitMask)> = Vec::new();
            let mut i: usize = 0;
            for row in 0..3 {
                for col in 0..3 {
                    let r_idx = r + row;
                    let c_idx = c + col;
                    if let Cell::Candidates(bits) = board[r_idx][c_idx] {
                        candidate_set.push((i, bits));
                    }
                    i += 1;
                }
            }
            let results = get_hidden_singles_from_masks(&candidate_set);
            for (idx, value) in results {
                let row = r + idx / 3;
                let col = c + idx % 3;
                board[row][col] = Cell::Value(value);
                change = true;
                local_change = true;
            } 
            if local_change {
                eliminate_candidates(board);
            }
            local_change = false;
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
    fn unique_candidate_finds_one_bit_only() {
        assert_eq!(get_index_of_unique_candidate(&mask(&[4])), Some(3));
        assert_eq!(get_index_of_unique_candidate(&mask(&[4, 7])), None);
        assert_eq!(get_index_of_unique_candidate(&mask(&[])), None);
    }

    #[test]
    fn hidden_single_helper_returns_value_and_cell_index() {
        let candidates = vec![
            (2, mask(&[1, 3])),
            (5, mask(&[1, 7])),
            (8, mask(&[1, 3])),
        ];

        assert_eq!(get_hidden_singles_from_masks(&candidates), vec![(5, 7)]);
    }

    #[test]
    fn hidden_single_helper_ignores_values_seen_more_than_once() {
        let candidates = vec![(0, mask(&[2, 8])), (4, mask(&[2, 8]))];

        assert!(get_hidden_singles_from_masks(&candidates).is_empty());
    }

    #[test]
    fn solves_a_hidden_single_in_a_row() {
        let mut board = candidate_board();
        board[3][1] = Cell::Candidates(mask(&[2, 5]));
        board[3][4] = Cell::Candidates(mask(&[2, 8]));
        board[3][7] = Cell::Candidates(mask(&[2, 5]));

        assert!(solve_hidden_singles(&mut board));
        assert!(matches!(board[3][4], Cell::Value(8)));
    }

    #[test]
    fn solves_a_hidden_single_in_a_column() {
        let mut board = candidate_board();
        board[1][6] = Cell::Candidates(mask(&[3, 4]));
        board[4][6] = Cell::Candidates(mask(&[3, 9]));
        board[7][6] = Cell::Candidates(mask(&[3, 4]));

        assert!(solve_hidden_singles(&mut board));
        assert!(matches!(board[4][6], Cell::Value(9)));
    }

    #[test]
    fn helper_avoids_conflicting_singles_for_the_same_cell() {
        let candidates = vec![
            (0, mask(&[1, 2, 3])),
            (1, mask(&[3, 4, 6])),
            (2, mask(&[3, 4, 6])),
        ];

        let results = get_hidden_singles_from_masks(&candidates);

        assert!(results.len() == 0);
    }

    #[test]
    fn solves_hidden_single_at_correct_position_in_box() {
        let mut board = candidate_board();

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
