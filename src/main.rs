mod view;
mod user_interaction;

use rand::Rng;
use view::{print_header, print_board};
use user_interaction::create_players;

fn main() {
    print_header();
    create_players();
    print_board();
}

fn get_random_number_from_range(bound: u16) -> u16 {
    rand::thread_rng().gen_range(0..bound)
}

