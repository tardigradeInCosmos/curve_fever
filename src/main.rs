mod user_interaction;
mod view;

use rand::Rng;
use user_interaction::create_players;
use view::{print_header, Board};

fn main() {
    print_header();
    create_players();
    let b = Board::build().unwrap();
    print!("{}", b);
}

fn get_random_number_from_range(bound: u16) -> u16 {
    rand::thread_rng().gen_range(0..bound)
}
