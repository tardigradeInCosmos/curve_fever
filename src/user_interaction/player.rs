use std::fmt;

#[derive(Debug, Clone)]
pub struct Player {
    left: String,
    right: String,
    designate: String,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.designate == other.designate
            || self.right == other.right
            || self.left == other.left
            || self.right == other.left
            || self.left == other.right
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "left key `{}`; right key `{}`; users designate `{}`",
            self.left, self.right, self.designate
        )
    }
}

impl Player {
    fn new(left: String, right: String, designate: String) -> Player {
        Player {
            left,
            right,
            designate,
        }
    }
    pub fn try_create(text: &str) -> Result<Player, String> {
        if text.len() <= 2 {
            return Err("To create user text must be 3 chars long".to_string());
        };
        if text.get(0..1) == text.get(1..2) {
            return Err("Can not use same key for both site steering".to_string());
        };

        let mut left = String::new();
        let mut right = String::new();
        let mut designate = String::new();

        for (i, c) in text.chars().enumerate() {
            match i {
                0 => left.push(c),
                1 => right.push(c),
                2 => designate.push(c),
                _ => continue,
            }
        }

        Ok(Player::new(left, right, designate))
    }
    pub fn is_valid(some_player: &Self) -> bool {
        let (l, r, d) = (
            some_player.left.len(),
            some_player.right.len(),
            some_player.designate.len(),
        );
        some_player.left != some_player.right && 1 == l && l == r && r == d
    }
}

#[derive(Debug)]
pub struct PlayersBench {
    pub players: Vec<Player>,
}

impl PlayersBench {
    pub fn new() -> Self {
        let players: Vec<Player> = Vec::new();
        PlayersBench { players }
    }

    pub fn add_player(&mut self, player: Player) -> Result<u8, String> {
        if !Player::is_valid(&player) {
            return Err("invalid player".to_string());
        }
        if self.is_duplicate(&player) {
            return Err("user is overlappin else's keys".to_string());
        }

        self.players.push(player);
        self.players
            .len()
            .try_into()
            .map_err(|_| "players aggregator corrupted".to_string())
    }
    fn is_duplicate(&self, new_val: &Player) -> bool {
        let mut is_duplicated = false;
        for holding in self.players.iter() {
            is_duplicated = holding == new_val;
            if is_duplicated {
                break;
            };
        }

        is_duplicated
    }
    pub fn print_players_choice(&self) {
        for (index, player) in self.players.iter().enumerate() {
            println!("player no. {} choice is {}", index + 1, player);
        }
    }
}

#[cfg(test)]
#[path = "./test.rs"]
mod test;
