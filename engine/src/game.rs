use crate::{courtier_card::{build_deck, CourtierCard}, player::Player, queens_table::{self, QueensTable}};

pub struct Game {
    deck: Vec<CourtierCard>,
    players: Vec<Player>,
    queens_table: QueensTable,
    pub current_player_number: u8,
}

impl Game {
    fn init_game(player_names: Vec<String>) -> Result<Game, crate::courtier_card::Error> {
        let deck = build_deck(player_names.len() as u8)?;
        let players = Player::init_players(player_names);
        let queens_table = QueensTable::new();
        Ok(Game { deck, players, queens_table, current_player_number: 0 })
    }

    fn next_player(&mut self) {
        self.current_player_number = (0..self.players.len())
            .into_iter()
            .cycle()
            .next()
            .unwrap()
            .try_into()
            .unwrap();
    }
}

// let mut player_number = (0..4).iter().cycle()
// struct PlayerNumber {
//     current_player_number: u8,
//     players_number: u8,
// }

// impl Iterator for PlayerNumber {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item> {

//     }
// }
