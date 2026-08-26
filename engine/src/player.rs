#![allow(dead_code)]
use std::iter::zip;

use crate::CourtierCard;
use crate::HiddenSpiesPiles;
use crate::MissionCard;
use crate::game::EndedGame;
use crate::piles::RevealedSpiesPiles;
use crate::queens_table::FamiliesStatuses;

pub struct ActivePlayer {
    pub player_name: String,
    pub hand: Vec<CourtierCard>,
    pub domain: HiddenSpiesPiles,
    pub mission_cards: Option<[MissionCard; 2]>,
}

pub struct GameEndedPlayer {
    pub player_name: String,
    pub domain: RevealedSpiesPiles,
    mission_cards: [MissionCard; 2],
}

pub struct PlayerScores {
    pub domain_scores: i8,
    pub missions_scores: i8,
}

impl ActivePlayer {
    pub fn init_players(player_names: Vec<String>) -> Vec<ActivePlayer> {
        let mut players: Vec<ActivePlayer> = Vec::new();
        for player_name in player_names {
            players.push(ActivePlayer {
                player_name: player_name,
                hand: Vec::new(),
                domain: HiddenSpiesPiles::new(),
                mission_cards: None,
            });
        }
        players
    }

    pub fn draw_hand(&mut self, deck: &mut Vec<CourtierCard>) {
        for _ in 0..=3 {
            self.hand.push(deck.pop().unwrap());
        }
    }

    pub fn play_card(&mut self, card: CourtierCard, piles: &mut HiddenSpiesPiles) {
        piles.add(card);
    }

    pub fn end_game(self) -> GameEndedPlayer {
        GameEndedPlayer {
            player_name: self.player_name,
            domain: self.domain.reveal_spies(),
            mission_cards: self
                .mission_cards
                .expect("Mission cards should have been dealt by then."),
        }
    }
}

impl GameEndedPlayer {
    fn compute_player_scores(&self, game: &EndedGame, current_player_index: i8) -> PlayerScores {
        let domain_scores = self.compute_domain_score(&game.statuses());
        let missions_scores = self.compute_missions_score(game, current_player_index);

        PlayerScores {
            domain_scores,
            missions_scores,
        }
    }

    fn compute_domain_score(&self, statuses: &FamiliesStatuses) -> i8 {
        zip(self.domain.tally().as_array(), statuses.as_array())
            .map(|(domain_family, family_status)| domain_family as i8 * family_status)
            .sum::<i8>()
    }

    fn compute_missions_score(&self, game: &EndedGame, current_player_index: i8) -> i8 {
        self.mission_cards
            .as_ref()
            .iter()
            .map(|mission_card| {
                if mission_card.get_mission_checker()(game, current_player_index) {
                    3i8
                } else {
                    0i8
                }
            })
            .sum::<i8>()
    }
}

impl PlayerScores {
    pub fn total_score(&self) -> i8 {
        self.domain_scores + self.missions_scores
    }
}
