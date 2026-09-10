use std::fs;
use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle, ParallelProgressIterator};
use rayon::prelude::*;

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
}

#[derive(Clone, Copy)]
struct Data {
    board: Board,
    diff: f64
}

fn parse_line(line: &str) -> Data {
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
    Data{board, diff}
}

fn count_unsolved_cells(board: &Board) -> usize {
    board.iter().flatten().filter(|&x| matches!(x, Cell::Candidates(_))).count()
}

fn solve(data: &mut Data) -> (Option<u128>, Data) {
    let start = Instant::now();

    let mut board = &mut (data.board);

    // let mut iter = 0;
    let mut c = eliminate_candidates(&mut board);
    while c {
        c = false;
        c |= solve_singles(&mut board);
        c |= determine_naked_doubles(&mut board);
        c |= determine_hidden_doubles(&mut board);
        c |= solve_singles(&mut board);
        c |= eliminate_pointing_sets(&mut board);
        c |= box_line_reduction(&mut board, UnitMode::Row);
        c |= box_line_reduction(&mut board, UnitMode::Column);
        c |= solve_singles(&mut board);
        c |= determine_naked_subsets(&mut board);
        c |= swordfish_elimination(&mut board);
        c |= solve_singles(&mut board);
        // if c {
        //     iter += 1;
        // }
        if count_unsolved_cells(board) == 0 {
            break;
        }
    }
    let duration = start.elapsed().as_micros();

    if verify_board(board) {
        return (Some(duration), *data)
    }
    else {
        return (None, *data)
    }
}

pub fn test_performance(difficulty: &Difficulty) {
    let diff_text = match difficulty {
        Difficulty::Easy => "easy",
        Difficulty::Medium => "medium",
        Difficulty::Hard => "hard",
        Difficulty::Diabolical => "diabolical",
    };
    let path = format!("data/{diff_text}.txt");
    
    let file = fs::read_to_string(path)
        .expect("File could not be found");

    let mut last_failed_board = None;    

    let mut dataset: Vec<Data> = Vec::new();

    for line in file.lines() {
        dataset.push(parse_line(&line));
    }
    let num_boards = dataset.len();

    // 1. Create a ProgressBar manually with your known length
    let pb = ProgressBar::new(dataset.len() as u64);

    // 2. Define a template containing `{per_sec}` or `{it_per_sec}`
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:30.green/green}] {pos}/{len} ({per_sec})"
        )
        .unwrap()
        .progress_chars("#>-")
    );
        
    println!("Solving {num_boards} boards of {diff_text} difficulty");
    let results: Vec<(Option<u128>, Data)> = dataset
        .par_iter()
        .progress_with(pb.clone())
        .map(|&(mut item)| solve(&mut item))
        .collect();
    pb.finish();
    
    

    let mut fails: usize = 0;
    let mut solved: usize = 0;
    let mut fail_sum: f64 = 0.0;
    let mut rate_sum: u128 = 0;
    for (rate, data) in results {
        if rate.is_some() {
            solved += 1;
            rate_sum += rate.unwrap();
        }
        else {
            fails += 1;
            fail_sum += data.diff;
            last_failed_board = Some(data.board);
        }
    }


    if rate_sum == 0 {
        println!("No reported successes");
    } else {
        let avg_rate = rate_sum as f64 / solved as f64;
        println!("Average completion speed: {avg_rate:.2} µs");
    };
    
    if fails == 0 {
        println!("No reported failures")
    }
    else {
        let avg_fail = fail_sum / fails as f64;
        println!("Average failure difficulty: {avg_fail} across {fails} tests");
        let percent = 100.0 * ((num_boards - fails) as f32 / num_boards as f32);
        println!("Solved {percent:.2}% of puzzles");
        println!("Last failed board:");
        let failed_board: Grid = cells_to_grid(&last_failed_board.unwrap());
        println!("{failed_board}");
    }
}