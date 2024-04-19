mod terminal;

use std::fmt;
pub use crate::view::board::terminal::{Width, Height, Terminal, TerminalSizeAcquisitor};

pub struct Board {
    w: u16,
    h: u16
}

impl fmt::Display for Board {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         let printable_board = self.create_printable();
         write!(f, "{}", printable_board)
    }
}

impl Board {
    fn new(size: (Width, Height)) -> Board {
        let (Width(w), Height(h)) = size;
        println!("Your terminal is {} cols wide and {} lines tall", w, h);
        Board { w, h }
    }

    fn build_with(tsa: impl TerminalSizeAcquisitor) -> Option<Board> {
        match tsa.get() {
            None => return None,
            Some(size) => return Some(Self::new(size))
        }
    }

    pub fn build() -> Option<Board> {
        Self::build_with(Terminal {})
    }

    fn create_printable(&self) -> String {
        let width = self.w;
        let height = self.h;

        let mut buff = String::new(); 
        for i in 2..=height {
            for j in 1..=width {
                let sign = match (i, j) {
                    (2, _n) => '_',
                    (_m, 1) => '|',
                    (_m, n) if n == width => '|',
                    (m, _n) if m == height => '_',
                    _ => ' '
                };
                 buff.push(sign);
            }
            buff.push(0xA as char);
        }
        buff
    }
}

#[cfg(test)]
#[path = "./test.rs" ]
mod test; 
