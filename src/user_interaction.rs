use std::io::stdin;
use std::fmt;
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

impl fmt::Display for Player {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "left key `{}`; right key `{}`; users designate `{}`", self.left, self.right, self.designate)
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
    fn try_create(text: &str) -> Result<Player, String> {
        if text.len() <= 2 { return Err("To create user text must be 3 chars long".to_string()) };
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

pub struct PlayersBench {
    players: Vec<Player>
}

impl PlayersBench {
    fn new() -> Self {
        let players: Vec<Player> = Vec::new();
        PlayersBench { players }
    }

    fn add_player(&mut self, player: Player) -> Result<u8, String> {
        if self.is_duplicate(&player) { return Err("user is overlappin else's keys".to_string()) }

        self.players.push(player);
        self.players.len().try_into().or_else(|_| return Err("players aggregator corrupted".to_string()))
    }
    fn is_duplicate(&self, new_val: &Player) -> bool {
        let mut is_duplicated = false;
        for holding in self.players.iter() {
            is_duplicated = holding == new_val; 
            if is_duplicated  { break };
        }

        is_duplicated
    }
    fn print_players_choice(&self) {
        for (index, player) in self.players.iter().enumerate() {
            println!("player no. {} choice is {}", index+1, player);
        }
    }
}
// =========================PLAYERS CREATION FOR USERS========================

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
