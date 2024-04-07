use std::io::stdin;

struct Steering {
    left: String,
    right :String
}
impl PartialEq for Steering {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left || self.left == other.right || self.right == other.right || self.right == other.left 
    }
}
pub struct Player {
    steering: Steering,
    designate: String
}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.steering == other.steering || self.designate == other.designate
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

fn get_players_number() -> u8 {
    let players_count = loop {

        let num = get_input("How many players there's going to be (max 3)?");

        let num: u8 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if num > 0 && num <=3 { break num };
        continue;
    };
    players_count 
}

fn print_players_choice(players: &Vec<Player>) {
    for (index, player) in players.iter().enumerate() {
        println!("player no. {} choose key `{}` for left and `{}` for right and `{}` to designate self", index+1, player.steering.left, player.steering.right, player.designate);
    }
}

 pub fn create_players() -> Vec<Player> {

    let players_number: u8 = get_players_number();

    let mut players_steerings: Vec<Player> = Vec::new();

    for i in 0..players_number {
        println!("Set steering for {} player", i+1);
        let steering = loop {
            let steering = get_users_steering_charset();
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

fn get_users_steering_charset () -> Player {
    let mut answer = String::new();
    while answer.len() <= 2  {
        answer = get_input("enter 3 signs - first 2 will be your left|right steering keys, latter will identify you on a board");
    }
    let left = String::from(&answer[0..1]);
    let right = String::from(&answer[1..2]);
    let user = String::from(&answer[2..3]);

    Player {
        steering: Steering { left: left, right: right },
        designate: user
    }
}

