#![allow(dead_code)]
use std::iter::zip;
use std::ops::AddAssign;

use crate::CourtierCard;
use crate::MissionCard;
use crate::Piles;
use crate::game;
use crate::game::Game;
use crate::mission_card;
use crate::piles::PilesScores;
use crate::queens_table::FamiliesStatuses;

pub struct Player {
    pub player_name: String,
    pub hand: Vec<CourtierCard>,
    pub domain: Piles,
    pub mission_cards: Option<[MissionCard; 2]>,
    pub domain_scores: Option<PilesScores>,
    pub total_score: i8,
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
                total_score: 0i8,
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

    pub fn compute_domain_score(&mut self, statuses: &FamiliesStatuses) {
        self.total_score += zip(self.domain.tally().as_array(), statuses.as_array())
            .map(|(domain_family, family_status)| domain_family as i8 * family_status)
            .sum::<i8>()
    }

    pub fn compute_missions_score(&mut self, game: &Game) {
        self.total_score += self
            .mission_cards
            .as_ref()
            .unwrap()
            .iter()
            .map(|mission_card| {
                if mission_card.get_mission_checker()(game) {
                    3i8
                } else {
                    0i8
                }
            })
            .sum::<i8>()
    }
}
