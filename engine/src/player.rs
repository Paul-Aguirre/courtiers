use std::iter::zip;

use crate::CourtierCard;
use crate::MissionCard;
use crate::Piles;
use crate::queens_table::FamiliesStatuses;

pub struct Player {
    pub player_name: String,
    pub hand: Vec<CourtierCard>,
    pub domain: Piles,
    pub mission_cards: (MissionCard, MissionCard),
}

impl Player {
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
