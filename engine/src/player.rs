use std::iter::zip;

use crate::CourtierCard;
use crate::MissionCard;
use crate::Piles;
use crate::queens_table::FamiliesStatues;

pub struct Player {
    player_name: String,
    hand: Vec<CourtierCard>,
    domain: Piles,
    mission_cards: (MissionCard, MissionCard),
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

    pub fn compute_score(&self, statuses: FamiliesStatues) -> i8 {
        zip(self.domain.tally().as_array(), statuses.as_array())
            .map(|(domain_family, family_status)| domain_family as i8 * family_status)
            .sum()
    }
}
