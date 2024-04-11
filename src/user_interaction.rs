use std::io::stdin;
// ========================PLAYER IMPLEMENTATION==============================
pub struct Player {
    left: String,
    right: String,
    designate: String
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.designate == other.designate ||
        self.right == other.right || 
        self.left == other.left ||
        self.right == other.left ||
        self.left == other.right
    }
}

impl Player {
    fn new(left: String, right: String, designate: String) -> Player {
        Player {
            left,
            right,
            designate
        }
    }
    fn try_create(text: &str) -> Result<Player, &str> {
        if text.len() <= 2 { return Err("To create user text must be 3 chars long") };
        let mut left = String::new();
        let mut right = String::new();
        let mut designate = String::new();
        
        for (i, c) in text.chars().enumerate() {
            match i {
                0 => left.push(c),
                1 => right.push(c),
                2 => designate.push(c),
                _ => continue
            }
        }

        Ok(Player::new(left, right, designate))
    }
    fn is_valid(some_player: &Self) -> bool {
        some_player.left != some_player.right
    }
}
// =========================PLAYERS CREATION FOR USERS========================

pub fn create_players() -> Vec<Player> {

    let players_number: u8 = get_players_number();

    let mut players_steerings: Vec<Player> = Vec::new();

    for i in 0..players_number {
        println!("Set steering for {} player", i+1);
        let steering = loop {
            let steering = create_player();
            if !is_duplicate(&players_steerings, &steering) { break steering }
        };
        players_steerings.push(steering);
    }
    print_players_choice(&players_steerings);

    players_steerings
}

fn is_duplicate(holder: &Vec<Player>, new_val: &Player) -> bool {
    let mut is_duplicated = false;
    for holding in holder.iter() {
        is_duplicated = holding == new_val; 
        if is_duplicated  { break };
    }

    is_duplicated
}

fn print_players_choice(players: &Vec<Player>) {
    for (index, player) in players.iter().enumerate() {
        println!("player no. {} choose key `{}` for left and `{}` for right and `{}` to designate self", index+1, player.left, player.right, player.designate);
    }
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

    let players_count = ask_until(prompt, to_number, eligible_user_amount_condition);

    players_count 
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
