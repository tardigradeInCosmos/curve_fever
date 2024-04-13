mod view;
mod user_interaction;

use rand::Rng;
use view::{Board, print_header};
use user_interaction::create_players;

fn main() {
    print_header();
    create_players();
    let b = Board::new();
    print!("{}", b);
}

fn get_random_number_from_range(bound: u16) -> u16 {
    rand::thread_rng().gen_range(0..bound)
}

