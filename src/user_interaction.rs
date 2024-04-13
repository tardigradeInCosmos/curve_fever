mod player;

use crate::user_interaction::player::{Player, PlayersBench};
use std::io::stdin;

pub fn create_players() -> PlayersBench {
    let mut players_bench = PlayersBench::new();
    let players_number: u8 = get_players_number();

    for i in 0..players_number {
        loop {
            println!("Set steering for {} player", i+1);
            let player = create_player();
            match players_bench.add_player(player) {
                Ok(players_amount) => {
                    println!("successfully added player, amount of players: {}", players_amount);
                    break
                 },
                 Err(msg) => {
                    println!("could not add player due to {}", msg);
                    continue
                 }
            }
        }
    }
    players_bench.print_players_choice();
    players_bench
}


fn create_player ()-> Player {
    let prompt = "enter 3 signs - first 2 will be your left|right steering keys, latter will identify you on a board";
    ask_until(prompt, Player::try_create, Player::is_valid)
}


fn get_players_number() -> u8 {
    let to_number = |x: &str| { x.trim().parse::<u8>() };
    let eligible_user_amount_condition = |num: &u8| { 
        let max_users = 3;
        *num > 0 && *num <= max_users
    };
    let prompt = "How many players there's going to be (max 3)?";

    ask_until(prompt, to_number, eligible_user_amount_condition)
}

fn ask_until<T, E, P, C>(prompt: &str, parser: P, condition: C) -> T 
    where
        P: Fn(&str) -> Result<T, E>,
        C: Fn(&T) -> bool
    {
           loop {
                let val = get_input(prompt);
                let formated = parser(&val);
                
                let to_check = match formated {
                    Ok(check) => check,
                    Err(_) => continue
                };
                
                if condition(&to_check) { break to_check }
           }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut answer = String::new();
    stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

     answer
}
