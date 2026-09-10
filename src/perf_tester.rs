use core::num;
use std::{fs, env};
use std::time::Instant;

use tqdm::tqdm;

use crate::box_line::*;
use crate::candidates::*;
use crate::helpers::*;
use crate::pairs::*;
use crate::singles::*;
use crate::subsets::*;
use crate::swordfish::*;
use crate::types::*;

pub enum Difficulty {
    Easy,
    Medium, 
    Hard,
    Diabolical,
    All
}

fn parse_line(line: &str) -> (Board, f64) {
    let mut board: Board = [[Cell::Empty; 9]; 9];
    let values: Vec<&str> = line.split_whitespace().collect();
    let mut idx = 0;
    debug_assert!(values[1].len() == 81);

    for digit in values[1].chars() {
        let num = digit.to_digit(10).unwrap() as usize;
        if num != 0 {
            board[idx / 9][idx % 9] = Cell::Value(num);
        }
        else {
            board[idx / 9][idx % 9] = Cell::Candidates([true; 9]);
        }
        idx += 1;
    }

    let diff: f64 = values[2].parse().unwrap(); 
    (board, diff)
}

fn count_unsolved_cells(board: &Board) -> usize {
    board.iter().flatten().filter(|&x| matches!(x, Cell::Candidates(_))).count()
}

fn solve(board: &mut Board) -> Option<u128>{
    let start = Instant::now();

    // let mut iter = 0;
    let mut c = eliminate_candidates(board);
    while c {
        c = false;
        c |= solve_singles(board);
        c |= determine_naked_doubles(board);
        c |= determine_hidden_doubles(board);
        c |= solve_singles(board);
        c |= eliminate_pointing_sets(board);
        c |= box_line_reduction(board, UnitMode::Row);
        c |= box_line_reduction(board, UnitMode::Column);
        c |= solve_singles(board);
        c |= determine_naked_subsets(board);
        c |= swordfish_elimination(board);
        c |= solve_singles(board);
        // if c {
        //     iter += 1;
        // }
        if count_unsolved_cells(board) == 0 {
            break;
        }
    }
    let duration = start.elapsed().as_micros();

    if verify_board(board) {
        return Some(duration)
    }
    else {
        return None
    }
}

pub fn test_performance(difficulty: &Difficulty) {
    let diff_text = match difficulty {
        Difficulty::Easy => "easy",
        Difficulty::Medium => "medium",
        Difficulty::Hard => "hard",
        Difficulty::Diabolical => "diabolical",

        // Solver is not strong enough to bother running diabolical or all
        Difficulty::All => "easy"
    };
    let path = format!("data/{diff_text}.txt");
    
    let file = fs::read_to_string(path)
        .expect("File could not be found");

    let mut last_failed_board = None;
    let mut rates: Vec<u128> = Vec::new();
    let mut fails: Vec<f64> = Vec::new();
    let mut num_boards = 0;
    for line in tqdm(file.lines()) {
        let (mut board, diff) = parse_line(&line);
        let perf = solve(&mut board);
        if perf.is_some() {
            rates.push(perf.unwrap());
        }
        else {
            fails.push(diff);
            last_failed_board = Some(board.clone());
        }
        num_boards += 1;
    }

    if rates.is_empty() {
        println!("No reported successes");
    } else {
        let sum: u128 = rates.iter().sum();
        let avg_rate = sum as f64 / rates.len() as f64;
        println!("Average completion speed: {avg_rate} µs");
    };
    
    if fails.is_empty() {
        println!("No reported failures")
    }
    else {
        let num_fails = fails.len();
        let sum: f64 = fails.iter().sum();
        let avg_fail = sum as f64 / fails.len() as f64;
        println!("Average failure difficulty: {avg_fail} across {num_fails} tests");
        let percent = 100.0 * ((num_boards - num_fails) as f32 / num_boards as f32);
        println!("Solved {percent:.2}% of puzzles");
        println!("Last failed board:");
        let failed_board: Grid = cells_to_grid(&last_failed_board.unwrap());
        println!("{failed_board}");
    }
}