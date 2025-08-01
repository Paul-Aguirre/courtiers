use std::iter::zip;

use crate::piles::PilesScores;
use crate::queens_table::FamiliesStatuses;
use crate::CourtierCard;
use crate::MissionCard;
use crate::Piles;

pub struct Player {
    pub player_name: String,
    pub hand: Vec<CourtierCard>,
    pub domain: Piles,
    pub mission_cards: Option<(MissionCard, MissionCard)>,
    pub domain_scores: Option<PilesScores>,
}

impl Player {
    pub fn init_players(player_names: Vec<String>) -> Vec<Player> {
        let mut players: Vec<Player> = Vec::new();
        for player_name in player_names {
            players.push(Player {
                player_name: player_name,
                hand: Vec::new(),
                domain: Piles::new(),
                mission_cards: None,
                domain_scores: None,
            });
        }
        players
    }

    pub fn draw_hand(&mut self, deck: &mut Vec<CourtierCard>) {
        for _ in 0..=3 {
            self.hand.push(deck.pop().unwrap());
        }
    }

    pub fn play_card(&mut self, card: &CourtierCard, piles: Piles) {
        todo!()
    }

    pub fn compute_score(&self, statuses: FamiliesStatuses) -> i8 {
        zip(self.domain.tally().as_array(), statuses.as_array())
            .map(|(domain_family, family_status)| domain_family as i8 * family_status)
            .sum()
    }
}
