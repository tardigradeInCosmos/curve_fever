use std::fmt;

#[derive(Debug, Clone)]
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
    pub fn try_create(text: &str) -> Result<Player, String> {
        if text.len() <= 2 { return Err("To create user text must be 3 chars long".to_string()) };
        if text.get(0..1) == text.get(1..2) { return Err("Can not use same key for both site steering".to_string()) };

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
        let (l, r, d) = (some_player.left.len(), some_player.right.len(), some_player.designate.len());
        some_player.left != some_player.right && 1 == l && l == r && r == d
    }
}

#[derive(Debug)] 
pub struct PlayersBench {
    players: Vec<Player>
}

impl PlayersBench {
    pub fn new() -> Self {
        let players: Vec<Player> = Vec::new();
        PlayersBench { players }
    }

    pub fn add_player(&mut self, player: Player) -> Result<u8, String> {
        if !Player::is_valid(&player) { return Err("invalid player".to_string()) }
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
    pub fn print_players_choice(&self) {
        for (index, player) in self.players.iter().enumerate() {
            println!("player no. {} choice is {}", index+1, player);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // PLAYER
    #[test]
    fn player_try_create_should_result_with_error_if_string_shorter_than_3_0() {
        assert_eq!(Player::try_create(""), Err("To create user text must be 3 chars long".to_string()));
    }
    #[test]
    fn player_try_create_should_result_with_error_if_string_shorter_than_3_1() {
        assert_eq!(Player::try_create("1"), Err("To create user text must be 3 chars long".to_string()));
    }
    #[test]
    fn player_try_create_should_result_with_error_if_string_shorter_than_3_2() {
        assert_eq!(Player::try_create("12"), Err("To create user text must be 3 chars long".to_string()));
    }
    #[test]
    fn player_try_create_should_result_ok_given_3_or_more_chars() {
        assert_eq!(Player::try_create("123"), Ok(Player {left: "1".to_string(), right: "2".to_string(), designate: "3".to_string()}));
    }
    #[test]
    fn player_try_create_should_result_error_given_string_with_duplicates_on_2_first_places() {
        assert_eq!(Player::try_create("111"), Err("Can not use same key for both site steering".to_string()));
    }
    #[test]
    fn player_is_valid_truthy_when_steerings_different_0() {
        let (left, right, designate) = ("a".to_string(), "b".to_string(), "b".to_string() );
        assert_eq!(Player::is_valid(&Player { left, right, designate }), true);
    }
    #[test]
    fn player_is_valid_truthy_when_steerings_different_1() {
        let (left, right, designate) = ("a".to_string(), "b".to_string(), "a".to_string() );
        assert_eq!(Player::is_valid(&Player { left, right, designate }), true);
    }
    #[test]
    fn player_is_valid_truthy_when_steerings_different_2() {
        let (left, right, designate) = ("a".to_string(), "b".to_string(), "c".to_string() );
        assert_eq!(Player::is_valid(&Player { left, right, designate }), true);
    }
    #[test]
    fn player_is_valid_falsy_when_steerings_same() {
        let (left, right, designate) = ("a".to_string(), "a".to_string(), "c".to_string());
        assert_eq!(Player::is_valid(&Player { left, right, designate }), false);
    }
    #[test]
    fn player_is_valid_falsy_when_multiletters_in_steering_0() {
        let (left, right, designate) = ("aa".to_string(), "a".to_string(), "c".to_string());
        assert_eq!(Player::is_valid(&Player { left, right, designate }), false);
    }
    #[test]
    fn player_is_valid_falsy_when_multiletters_in_steering_1() {
        let (left, right, designate) = ("a".to_string(), "ax".to_string(), "c".to_string());
        assert_eq!(Player::is_valid(&Player { left, right, designate }), false);
    }
    #[test]
    fn player_is_valid_falsy_when_multiletters_in_steering_2() {
        let (left, right, designate) = ("a".to_string(), "a".to_string(), "cd".to_string());
        assert_eq!(Player::is_valid(&Player { left, right, designate }), false);
    }
    #[test]
    fn player_to_string() {
        let (left, right, designate) = ("a".to_string(), "b".to_string(), "c".to_string() );
        let player = Player { left, right, designate };
        assert_eq!(player.to_string(), "left key `a`; right key `b`; users designate `c`".to_string());
    }
    //PlayersBench
    #[test]
    fn players_bench_new_creates_object_with_empty_vec_of_player() {
        let bench = PlayersBench::new();
        assert_eq!(bench.players, Vec::<Player>::new());
    }
    #[test]
    fn players_bench_succesfull_add_player_result_with_number_of_players_0() {
        assert_eq!(PlayersBench::new().add_player(Player { left: String::from("l"), right: String::from("r"), designate: String::from("d")}), Ok(1u8));
    }
    #[test]
    fn players_bench_succesfull_add_player_result_with_number_of_players_1() {
        let mut bench = PlayersBench::new();
        let player_1 = Player { left: String::from("l"), right: String::from("r"), designate: String::from("d")};
        let player_2 = Player { left: String::from("a"), right: String::from("b"), designate: String::from("c")};
        let player_3 = Player { left: String::from("x"), right: String::from("y"), designate: String::from("z")};
        assert_eq!(bench.add_player(player_1), Ok(1u8));
        assert_eq!(bench.add_player(player_2), Ok(2u8));
        assert_eq!(bench.add_player(player_3), Ok(3u8));
    }
    #[test]
    fn players_bench_error_add_player_result_when_duplicated_player() {
        let mut bench = PlayersBench::new();
        let player_1 = Player { left: String::from("l"), right: String::from("r"), designate: String::from("d")};
        let player_1_clone = player_1.clone();
        assert_eq!(bench.add_player(player_1), Ok(1u8));
        assert_eq!(bench.add_player(player_1_clone), Err("user is overlappin else's keys".to_string()));
    }
    #[test]
    fn players_bench_add_player_after_error_can_add_more_valid_player() {
        let mut bench = PlayersBench::new();
        let player_1 = Player { left: String::from("l"), right: String::from("r"), designate: String::from("d")};
        let player_1_clone = player_1.clone();
        assert_eq!(bench.add_player(player_1), Ok(1u8));
        assert!(bench.add_player(player_1_clone).is_err());
        assert_eq!(bench.add_player(Player { left: String::from("a"), right: String::from("b"), designate: String::from("c")}), Ok(2u8));
    }
    #[test]
    fn players_bench_error_add_player_result_when_invalid_player_0() {
        assert_eq!(PlayersBench::new().add_player(Player { left: String::from("ll"), right: String::from("r"), designate: String::from("d")}), Err("invalid player".to_string()));
    }
    #[test]
    fn players_bench_error_add_player_result_when_invalid_player_1() {
        assert!(PlayersBench::new().add_player(Player { left: String::from("l"), right: String::from("l"), designate: String::from("d")}).is_err());
    }
    #[test]
    fn players_bench_add_player_after_error_can_add_valid_player() {
        let mut bench = PlayersBench::new();
        assert_eq!(bench.add_player(Player { left: String::from("ll"), right: String::from("r"), designate: String::from("d")}), Err("invalid player".to_string()));
        assert_eq!(bench.add_player(Player { left: String::from("l"), right: String::from("r"), designate: String::from("d")}), Ok(1u8));
    }
}
