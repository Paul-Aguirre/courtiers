#![allow(dead_code)]
use std::{cell::RefCell, iter::Cycle, ops::Range, rc::Rc};

use crate::{
    courtier_card::{self, CourtierCard},
    player::{self, Player},
    queens_table::QueensTable,
};

pub struct Game {
    deck: Vec<CourtierCard>,
    players: Vec<Player>,
    queens_table: QueensTable,
    current_player_index: u8,
    player_cycle_iter: Box<Cycle<Range<usize>>>,
}

impl Game {
    pub fn init_game(player_names: Vec<String>) -> Result<Game, crate::courtier_card::Error> {
        // initialize deck of shuffle CourtierCard instances
        let deck = courtier_card::build_deck(player_names.len() as u8)?;

        // initialize Vec<Player> with empty hand, domain, mission_cards, and domain_scores
        let players = Player::init_players(player_names);

        // distribute cards for a hand to each player
        // ...
        // distribute 2 MissionCard instances to each player
        // ...

        // initialize QueensTable instance with empty Piles instances and statuses
        let queens_table = QueensTable::new();

        let mut player_cycle_iter = Box::new((0..players.len()).into_iter().cycle());

        Ok(Game {
            deck,
            players,
            queens_table,
            current_player_index: player_cycle_iter.next().unwrap().try_into().unwrap(),
            player_cycle_iter,
        })
    }

    pub fn start_main_loop(&mut self) {
        todo!()
    }

    pub fn end_game(&mut self) {
        // determine family statuses
        self.queens_table.determine_family_statuses();

        // tally points from players domains
        for player in &mut self.players {
            player.compute_domain_score(self.queens_table.get_statuses().as_ref().unwrap());
        }

        // tally points from players mission cards
        // TODO test this part, I'm unsure it'll work
        let wrapped_game = Rc::new(RefCell::new(self));
        for player in &mut wrapped_game.borrow_mut().players {
            player.compute_missions_score(&wrapped_game.borrow_mut());
        }

        // determine and print the winner / total scores / score board
    }

    fn next_player(&mut self) {
        self.current_player_index = self.player_cycle_iter.next().unwrap().try_into().unwrap();
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
    fn next_player_call_increments_current_player_index() {
        let player_names = vec![
            String::from("Bob"),
            String::from("Alice"),
            String::from("Guido"),
        ];
        let mut game = Game::init_game(player_names).unwrap();
        assert_eq!(game.current_player_index, 0);
        game.next_player();
        assert_eq!(game.current_player_index, 1);
        game.next_player();
        assert_eq!(game.current_player_index, 2);
        game.next_player();
        assert_eq!(game.current_player_index, 0);
    }

    #[test]
    #[ignore = "Not yet implemented"]
    fn test_end_game() {
        todo!()
        // determine family statuses
        // tally points from players domains
        // tally points from players mission cards
        // determine and print the winner / total scores / score board
    }
}
