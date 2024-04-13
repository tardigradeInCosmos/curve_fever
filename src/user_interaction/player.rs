use std::fmt;

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
    pub fn new(left: String, right: String, designate: String) -> Player {
        Player {
            left,
            right,
            designate
        }
    }
    pub fn try_create(text: &str) -> Result<Player, String> {
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
    pub fn is_valid(some_player: &Self) -> bool {
        some_player.left != some_player.right
    }
}

pub struct PlayersBench {
    players: Vec<Player>
}

impl PlayersBench {
    pub fn new() -> Self {
        let players: Vec<Player> = Vec::new();
        PlayersBench { players }
    }

    pub fn add_player(&mut self, player: Player) -> Result<u8, String> {
        if self.is_duplicate(&player) { return Err("user is overlappin else's keys".to_string()) }

        self.players.push(player);
        self.players.len().try_into().or_else(|_| return Err("players aggregator corrupted".to_string()))
    }
    pub fn is_duplicate(&self, new_val: &Player) -> bool {
        let mut is_duplicated = false;
        for holding in self.players.iter() {
            is_duplicated = holding == new_val; 
            if is_duplicated  { break };
        }

        is_duplicated
    }
    pub fn print_players_choice(&self) {
        for (index, player) in self.players.iter().enumerate() {
            println!("player no. {} choice is {}", index+1, player);
        }
    }
}
