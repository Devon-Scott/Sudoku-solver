use std::fmt;

// Want to test performance differences between using BitArray and array of bools
pub type BitMask = [bool; 9];

pub trait BitMaskExt {
    fn bits_set(&self) -> usize;
    // fn contains(&self, other: &BitMask) -> bool;
    // fn remove(&mut self, other: &BitMask);
}

impl BitMaskExt for BitMask {
    fn bits_set(&self) -> usize {
        self.iter().filter(|x| **x == true).count()
    }

    // fn contains(&self, other: &BitMask) -> bool {
    //     for i in 0..self.len() {
    //         if self[i] && !other[i] {
    //             return false
    //         }
    //     }
    //     let larger = self.bits_set() > other.bits_set();
    //     larger
    // }

    // fn remove(&mut self, other: &BitMask) {
    //     for i in 0..self.len() {
    //         if self[i] != !other[i] {
    //             self[i] = false;
    //         }
    //     }
    // }
}

#[derive(PartialEq, Debug)]
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
                let _ = write!(f, "{col} ");
                c += 1;
            }
            let _ = writeln!(f, "|");
            r += 1;
        }
        writeln!(f, "{horizontal_bar}")
    }
}