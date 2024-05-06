#[warn(unused_imports)]
extern crate sdl2; 

mod user_interaction;
mod view;

use std::thread;
use std::time::Duration;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use user_interaction::{create_players, Player, PlayersBench};
use view::{print_header, Board};

fn main() {
    print_header();
    let players_bench: PlayersBench = create_players();
    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    players_bench.players.iter().for_each( |player: &Player| {
        let handle = thread::spawn(|| {
            'running: loop {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit {..} |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            break 'running
                        },
                        _ => {}
                    }
                }
            }
        });
        &mut threads.push(handle);
    });

    threads.iter().for_each(|th| th.join().unwrap() );
    
    // let b = Board::build().unwrap();
    // print!("{}", b);
}

fn get_random_number_from_range(bound: u16) -> u16 {
    rand::thread_rng().gen_range(0..bound)
}
