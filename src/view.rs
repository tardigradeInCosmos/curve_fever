use terminal_size::{Width, Height, terminal_size};

struct Board {
    w: u16,
    h: u16
}

pub fn print_board() {
    let board = get_terminal_size();
    let rows = board.h;
    let cols = board.w;
    print_rectangle(cols, rows);
}

fn print_rectangle(width: u16, height: u16) {

    for i in 2..=height {
        let mut printable_row = String::new().to_owned();
        for j in 1..=width {
            let sign = match (i, j) {
                (2, _n) => '_',
                (_m, 1) => '|',
                (_m, n) if n == width => '|',
                (m, _n) if m == height => '_',
                _ => ' '
            };
        printable_row.push(sign);
        }
        println!("{}", printable_row);
    }
}

fn get_terminal_size() -> Board {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        println!("Your terminal is {} cols wide and {} lines tall", w, h);
        Board { w: w, h: h }
    } else {
        println!("Unable to get terminal size");
        Board { w: 10, h: 10 }
    }
}    

pub fn print_header() {
    println!("*********************");
    println!("*********************");
    println!("**   curve fever   **");
    println!("*********************");
    println!("*********************");
}

