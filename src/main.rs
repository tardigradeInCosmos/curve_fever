#[warn(unused_imports)]
extern crate sdl3; 

mod user_interaction;
mod view;

use std::thread;
use std::time::Duration;
use rand::Rng;
use user_interaction::{create_players, Player, PlayersBench};
use view::{print_header, Board};

fn main() {
    /*print_header();
    let players_bench: PlayersBench = create_players();
    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();
    let sdl_context = sdl3::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let b = Board::build().unwrap();
    print!("{}", b);*/
}

fn get_random_number_from_range(bound: u16) -> u16 {
    rand::thread_rng().gen_range(0..bound)
}
