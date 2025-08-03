use crate::{
    courtier_card::{CourtierCard, build_deck},
    player::Player,
    queens_table::{self, QueensTable},
};

pub struct Game {
    deck: Vec<CourtierCard>,
    players: Vec<Player>,
    queens_table: QueensTable,
    current_player_index: u8,
}

impl Game {
    fn init_game(player_names: Vec<String>) -> Result<Game, crate::courtier_card::Error> {
        let deck = build_deck(player_names.len() as u8)?;
        let players = Player::init_players(player_names);
        let queens_table = QueensTable::new();
        Ok(Game {
            deck,
            players,
            queens_table,
            current_player_index: 0,
        })
    }

    fn next_player(&mut self) {
        // TODO test this one !!!!!!!!!!!!
        self.current_player_index = (0..self.players.len())
            .into_iter()
            .cycle()
            .next()
            .unwrap()
            .try_into()
            .unwrap();
    }

    pub fn get_current_player(&self) -> &Player {
        &self.players[self.current_player_index as usize]
    }

    pub fn get_next_player(&self) -> &Player {
        if self.current_player_index == self.players.len() as u8 - 1 {
            &self.players[0]
        } else {
            &self.players[self.current_player_index as usize + 1]
        }
    }

    pub fn get_queens_table(&self) -> &QueensTable {
        &self.queens_table
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_player_increments_current_player_index() {
        todo!()
    }
}
