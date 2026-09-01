type Board = [[i16;9]; 9];

const TEST_GRID: Board = [
    [0, 0, 0, 0, 0, 8, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 9, 0, 0],
    [3, 0, 4, 9, 0, 0, 8, 2, 0],
    [7, 0, 0, 0, 0, 2, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 2, 0, 0],
    [0, 0, 9, 0, 1, 6, 0, 7, 0],
    [0, 0, 3, 0, 6, 0, 0, 0, 1],
    [0, 6, 1, 0, 8, 3, 0, 4, 0],
    [4, 0, 0, 5, 0, 0, 0, 6, 0]]; 

const SOLVED_GRID: Board = [
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

fn verify(board: &Board) -> bool {
    for row in 0..9 {
        let mut row_checks: [bool; 9] = [false; 9];
        for col in 0..9 {
            let num: usize = board[row][col] as usize;
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
            let num: usize = board[row][col] as usize;
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
                    let num: usize = board[r][c] as usize;
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


fn check_rows_cols(board: &mut Board) -> bool {
    for n in 1..10 {
        
    };
    false
}

fn main() {
    let board: Board = TEST_GRID;
    if !verify(&board) {
        print!("Board is not solved\n")
    }
    if verify(&SOLVED_GRID) {
        print!("Solved board is solved\n")
    }
}
