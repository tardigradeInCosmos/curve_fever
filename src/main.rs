use std::io::stdin;
use terminal_size::{Width, Height, terminal_size};
use rand::Rng;

fn main() {
    print_header();
    let players = create_players();
    print_players_choice(&players);
    print_board();
}

fn print_board() {
    let board_edges = get_board_size();
    for i in [0..board_edges.0] {

        for j in [0..board_edges.1] {

        }
    }

fn get_board_size() -> (u16, u16) {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        println!("Your terminal is {} cols wide and {} lines tall", w, h);
        (w, h)
    } else {
        println!("Unable to get terminal size");
        (10, 10)
    }
}    

fn print_players_choice(players: &Vec<(String, String, String)>) {
    for (index, player) in players.iter().enumerate() {
        println!("player no. {} choose key `{}` for left and `{}` for right and `{}` to designate self", index+1, player.0, player.1, player.2);
    }
}

fn create_players() -> Vec<(String, String, String)> {

    let players_number: u8 = get_players_number();

    let mut players_steerings: Vec<(String, String, String)> = Vec::new();

    for i in 0..players_number {
        println!("Set steering for {} player", i+1);
        let steering = loop {
            let steering = get_users_steering_charset();
            if !is_duplicate(&players_steerings, &steering) { break steering }
        };
        players_steerings.push(steering);
    }

    players_steerings
}

fn get_players_number() -> u8 {
    let players_count = loop {
   
        let num = get_input("How many players there's going to be?");

        let num: u8 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break num;
    };
   players_count 
}

fn is_duplicate(holder: &Vec<(String, String, String)>, new_val: &(String, String, String)) -> bool {
    let mut is_duplicated = false;
    let (x, y, z) = new_val;
    for holding in holder.iter() {
        is_duplicated = 
            holding.0 == *x ||
            holding.1 == *x ||
            holding.0 == *y ||
            holding.1 == *y ||
            holding.2 == *z;
        if is_duplicated  { break };
    }

    is_duplicated
}

fn get_users_steering_charset () -> (String, String, String) {
    let mut answer = String::new();
    while answer.len() <= 2  {
        answer = get_input("enter 3 signs - first 2 will be your left|right steering keys, latter will identify you on a board");
    }
    let left = String::from(&answer[0..1]);
    let right = String::from(&answer[1..2]);
    let user = String::from(&answer[2..3]);

    (left, right, user)
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut answer = String::new();
    stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    answer
}

fn print_header() {
    println!("*********************");
    println!("*********************");
    println!("**   curve fever   **");
    println!("*********************");
    println!("*********************");
}

