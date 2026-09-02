mod singles;
mod types;

use crate::singles::*;
use crate::types::*;

const TEST_GRID: BasicGrid = [
    [0, 0, 0, 0, 0, 8, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 9, 0, 0],
    [3, 0, 4, 9, 0, 0, 8, 2, 0],
    [7, 0, 0, 0, 0, 2, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 2, 0, 0],
    [0, 0, 9, 0, 1, 6, 0, 7, 0],
    [0, 0, 3, 0, 6, 0, 0, 0, 1],
    [0, 6, 1, 0, 8, 3, 0, 4, 0],
    [4, 0, 0, 5, 0, 0, 0, 6, 0]];

const SOLVED_GRID: BasicGrid = [
    [1, 2, 3, 4, 5, 6, 7, 8, 9],
    [4, 5, 6, 7, 8, 9, 1, 2, 3],
    [7, 8, 9, 1, 2, 3, 4, 5, 6],
    [2, 3, 4, 5, 6, 7, 8, 9, 1],
    [5, 6, 7, 8, 9, 1, 2, 3, 4],
    [8, 9, 1, 2, 3, 4, 5, 6, 7],
    [3, 4, 5, 6, 7, 8, 9, 1, 2],
    [6, 7, 8, 9, 1, 2, 3, 4, 5],
    [9, 1, 2, 3, 4, 5, 6, 7, 8]
];

fn verify(grid: &BasicGrid) -> bool {
    for row in 0..9 {
        // Maybe use a bit vector and logical operators?
        let mut row_checks: [bool; 9] = [false; 9];
        for col in 0..9 {
            let num: usize = grid[row][col] as usize;
            if num == 0 {
                return false
            }
            row_checks[num - 1] = true
        }
        for i in 0..9{
            if row_checks[i] == false {
                return false
            }
        }
    }

    for col in 0..9 {
        let mut col_checks: [bool; 9] = [false; 9];
        for row in 0..9 {
            let num: usize = grid[row][col] as usize;
            if num == 0 {
                return false
            }
            col_checks[num - 1] = true
        }
        for i in 0..9{
            if col_checks[i] == false {
                return false
            }
        }
    }

    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut box_checks = [false; 9];
            for row in 0..3 {
                for col in 0..3 {
                    let r: usize = box_row * 3 + row;
                    let c: usize = box_col * 3 + col;
                    let num: usize = grid[r][c] as usize;
                    if num == 0 {
                        return false
                    }
                    box_checks[num - 1] = true;
                }
            }
            for i in 0..9{
                if box_checks[i] == false {
                    return false
                }
            }
        }
    }
    true
}

fn make_candidate_sets(board: &mut Board) {
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == Cell::Empty {
                board[row][col] = Cell::Candidates([true; 9])
            }
        }
    }
}

fn eliminate_candidates(board: &mut Board) -> bool {
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

// Takes in a vector of index and a bitmask, and if there is a hidden single at the index,
// returns that index, with the hidden single value
fn get_hidden_singles_from_masks(candidate_set: &Vec<(usize, BitMask)>) -> Vec<(usize, i16)> {
    let mut result: Vec<(usize, i16)> = Vec::new();
    for value in 0..9 {
        let mut seen = false;
        let mut single = true;
        let mut idx = 0;
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
    result
}

fn solve_hidden_singles(board: &mut Board) -> bool {
    let mut change = false;

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
        } 
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
        } 
    }
    
    // For box
    
    change
}

fn grid_to_cells(grid: &BasicGrid) -> Board {
    grid.map(|row| {
        row.map(|val| match val {
            0 => Cell::Empty,
            n => Cell::Value(n)
        })
    })
}

fn cells_to_grid(board: &Board) -> BasicGrid {
    let mut result: BasicGrid = [[0; 9]; 9];
    for row in 0..9 {
        for col in 0..9 {
            result[row][col] = match &board[row][col] {
                Cell::Value(num) => *num,
                _ => 0
            };
        }
    }
    result
}

fn main() {
    let grid: BasicGrid = TEST_GRID;
    let mut board: Board = grid_to_cells(&grid);
    if !verify(&grid) {
        println!("Board is not solved\n");
        println!("{}", Grid(grid));
    }
    if verify(&SOLVED_GRID) {
        println!("Solved board is solved\n");
    }

    println!("One iteration of candidate checks");
    make_candidate_sets(&mut board);

    let mut c = eliminate_candidates(&mut board);
    while c {
        c = false;
        c = c | solve_naked_singles(&mut board);
        c = c | eliminate_candidates(&mut board);
        c = c | solve_hidden_singles(&mut board);
        c = c | eliminate_candidates(&mut board);
    }
    println!("{}", Grid(cells_to_grid(&board)));


}
