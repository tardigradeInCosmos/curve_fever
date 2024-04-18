use std::fmt;
use terminal_size::{terminal_size, Width, Height};

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
    pub fn new() -> Board {
        let size = terminal_size();
        if let Some((Width(w), Height(h))) = size {
            println!("Your terminal is {} cols wide and {} lines tall", w, h);
            Board { w, h }
        } else {
            println!("Unable to get terminal size yout board is going to be 10x10");
            Board { w: 10, h: 10 }
        }
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
// #[path = "./test.rs" ]
mod test {
    use super::*;

    #[test]
    fn board_new_terminal_size_inaccesible() {
        let b = Board::new();
        assert_eq!(b.w, 10u16, "board width value expected to match 10, got {}", b.w);
        assert_eq!(b.h, 10u16, "board height value expected to match 10, got {}", b.h);
    }
} 
