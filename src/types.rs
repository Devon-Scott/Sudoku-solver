use std::fmt;

// Want to test performance differences between using BitArray and array of bools
pub type BitMask = [bool; 9];

pub trait BitMaskExt {
    fn bits_set(&self) -> usize;
    fn and_mask(lhs: &BitMask, rhs: &BitMask) -> BitMask;
    fn or(&mut self, rhs: &BitMask) -> BitMask;
    fn and(&mut self, rhs: &BitMask);
    fn remove(&mut self, other: &BitMask) -> bool;
}

impl BitMaskExt for BitMask {
    fn bits_set(&self) -> usize {
        self.iter().filter(|x| **x == true).count()
    }

    fn and_mask(lhs: &BitMask, rhs: &BitMask) -> BitMask {
        let mut result: BitMask = [false; 9];
        for value in 0..9 {
            if lhs[value] && rhs[value] {
                result[value] = true;
            }
        }
        result
    }

    fn or(&mut self, rhs: &BitMask) -> BitMask {
        for value in 0..self.len() {
            if rhs[value] {
                self[value] = true;
            }
        }
        *self
    }

    fn and(&mut self, rhs: &BitMask) {
        for value in 0..self.len() {
            if self[value] && rhs[value] {
                self[value] = true;
            }
            else {
                self[value] = false;
            }
        }
    }

    fn remove(&mut self, other: &BitMask) -> bool {
        let mut change = false;
        for i in 0..self.len() {
            if self[i] && other[i] {
                self[i] = false;
                change = true;
            }
        }
        change
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Candidates(BitMask),
    Value(i16)
}

pub type BasicGrid = [[i16;9]; 9];
pub struct Grid(pub BasicGrid);

pub type Board = [[Cell;9]; 9];

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let mut result: fmt::Formatter
        let horizontal_bar = " ------- ------- ------- ";
        let mut r = 0;
        for row in &self.0 {
            if r % 3 == 0 {
                let _ = writeln!(f, "{horizontal_bar}");
            }
            let mut c = 0;
            for col in row {
                if c % 3 == 0 {
                    let _ = write!(f, "| ");
                }
                if *col == 0 {
                    let _ = write!(f, ". ");
                }
                else {
                    let _ = write!(f, "{col} ");
                }
                
                c += 1;
            }
            let _ = writeln!(f, "|");
            r += 1;
        }
        writeln!(f, "{horizontal_bar}")
    }
}