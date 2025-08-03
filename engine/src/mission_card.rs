use std::cmp::Ordering;
use std::sync::Arc;

use rand::seq::SliceRandom;

use crate::{
    courtier_card::{CourtierFamily, CourtierRole},
    game::Game,
    mission_card::{
        blue_cards::{build_check_disgraced_card, count_all_piles, count_any_piles, count_status},
        white_cards::{build_cmp_neighbor_card, build_count_role_card},
    },
    queens_table::FamilyStatus,
};

#[derive(Clone)]
enum MissionCardColor {
    Blue,
    White,
}

pub struct MissionCard {
    color: MissionCardColor,
    text: String,
    mission_checker: Arc<dyn Fn(Game) -> bool + Send + Sync>,
}

// Manual Clone implementation for MissionCard
impl Clone for MissionCard {
    fn clone(&self) -> Self {
        MissionCard {
            color: self.color.clone(),
            text: self.text.clone(),
            mission_checker: Arc::clone(&self.mission_checker),
        }
    }
}

impl MissionCard {
    pub fn build_white_deck() -> Vec<MissionCard> {
        let mut cmp_cards: Vec<MissionCard> = Vec::new();

        for family in CourtierFamily::families_iter() {
            cmp_cards.push(build_cmp_neighbor_card(family));
        }

        let mut count_role_cards: Vec<MissionCard> = Vec::new();

        for role in CourtierRole::special_roles_iter() {
            count_role_cards.push(build_count_role_card(role));
        }

        let mut white_deck = [cmp_cards, count_role_cards].concat();
        white_deck.shuffle(&mut rand::rng());

        white_deck
    }

    pub fn build_blue_deck() -> Vec<MissionCard> {
        let mut check_disgraced_cards: Vec<MissionCard> = Vec::new();

        for family in CourtierFamily::families_iter() {
            check_disgraced_cards.push(build_check_disgraced_card(family));
        }

        let count_status_most_card = MissionCard {
            color: MissionCardColor::Blue,
            text: String::from("At most 3 families must be in the light at the court."),
            mission_checker: Arc::new(|game: Game| {
                count_status(
                    game.get_queens_table().get_statuses().as_ref().unwrap(),
                    FamilyStatus::Disgraced,
                ) <= 3
            }),
        };

        let count_status_least_card = MissionCard {
            color: MissionCardColor::Blue,
            text: String::from("At least 2 families must be disgraced at the court."),
            mission_checker: Arc::new(|game: Game| {
                count_status(
                    game.get_queens_table().get_statuses().as_ref().unwrap(),
                    FamilyStatus::Disgraced,
                ) >= 2
            }),
        };

        let count_all_piles_card = MissionCard {
            color: MissionCardColor::Blue,
            text: String::from("At least 1 card of each family must be under the play mat."),
            mission_checker: Arc::new(|game: Game| {
                count_all_piles(&game.get_queens_table().get_disgraced().tally(), 1)
            }),
        };

        let count_any_piles_card = MissionCard {
            color: MissionCardColor::Blue,
            text: String::from("One family must have at least 5 cards under the play mat."),
            mission_checker: Arc::new(|game: Game| {
                count_any_piles(&game.get_queens_table().get_disgraced().tally(), 5)
            }),
        };

        let mut blue_deck = [
            check_disgraced_cards,
            [
                [count_status_most_card],
                [count_status_least_card],
                [count_all_piles_card],
                [count_any_piles_card],
            ]
            .concat(),
        ]
        .concat();

        blue_deck.shuffle(&mut rand::rng());

        blue_deck
    }
}

mod white_cards {
    use std::{cmp::Ordering, sync::Arc};

    use crate::{
        courtier_card::{CourtierFamily, CourtierRole},
        game::Game,
        mission_card::{MissionCard, MissionCardColor},
        piles::{GetFamily, Piles, PilesScores},
    };

    // ################################################################
    // -----------------------cmp neighbor cards-----------------------
    // ################################################################
    pub fn build_cmp_neighbor_card(family: &'static CourtierFamily) -> MissionCard {
        MissionCard {
            color: MissionCardColor::White,
            text: build_cmp_card_text(family),
            mission_checker: cmp_neighbor_mission_checker(family),
        }
    }

    fn cmp_neighbor_mission_checker(
        family: &'static CourtierFamily,
    ) -> Arc<dyn Fn(Game) -> bool + Send + Sync> {
        fn inner(game: Game, family: &'static CourtierFamily) -> bool {
            cmp_neighbor(
                game.get_current_player().domain_scores.clone().unwrap(),
                game.get_next_player().domain_scores.clone().unwrap(),
                family,
            )
            .is_ge()
        }
        // Use a wrapper function to match the expected fn(Game) -> bool signature
        Arc::new(move |game: Game| inner(game, family))
    }

    fn cmp_neighbor(left: PilesScores, right: PilesScores, family: &CourtierFamily) -> Ordering {
        // compares the number of cards in a specified family between two PliesScores instancess
        left.get_family(family).cmp(right.get_family(family))
    }

    fn build_cmp_card_text(family: &CourtierFamily) -> String {
        format!("You must have less {}s than your left neighbor.", family)
    }

    // ################################################################
    // ------------------------count role cards------------------------
    // ################################################################
    pub fn build_count_role_card(role: &'static CourtierRole) -> MissionCard {
        MissionCard {
            color: MissionCardColor::White,
            text: build_count_role_card_text(role, get_min_for(role)),
            mission_checker: count_roles_mission_checker(role),
        }
    }

    fn count_role_across_families(piles: &Piles, role: &CourtierRole) -> u8 {
        // counts the number of cards with role across all the cards in a Piles instance
        Vec::from(piles.as_array())
            .into_iter()
            .flatten()
            .filter(|card| &card.role == role)
            .count()
            .try_into()
            .unwrap()
    }

    fn build_count_role_card_text(role: &CourtierRole, min: u8) -> String {
        format!("You must posess at least {} {}s.", min, role)
    }

    fn get_min_for(role: &CourtierRole) -> u8 {
        match role {
            CourtierRole::Assassin => Some(2u8),
            CourtierRole::Noble => Some(3u8),
            CourtierRole::Spy => Some(3u8),
            CourtierRole::Guard => Some(4u8),
            _ => None,
        }
        .unwrap()
    }

    fn count_roles_mission_checker(
        role: &'static CourtierRole,
    ) -> Arc<dyn Fn(Game) -> bool + Send + Sync> {
        fn inner(game: Game, role: &'static CourtierRole) -> bool {
            count_role_across_families(&game.get_current_player().domain, role)
                .cmp(&get_min_for(role))
                .is_ge()
        }
        Arc::new(move |game: Game| inner(game, role))
    }
}

mod blue_cards {
    use std::sync::Arc;

    use crate::{
        courtier_card::CourtierFamily,
        game::{self, Game},
        mission_card::MissionCard,
        piles::{GetFamily, PilesScores},
        queens_table::{FamiliesStatuses, FamilyStatus},
    };

    // ################################################################
    // ----------------------check disgraced cards----------------------
    // ################################################################
    pub fn build_check_disgraced_card(family: &'static CourtierFamily) -> MissionCard {
        MissionCard {
            color: super::MissionCardColor::Blue,
            text: build_check_disgraced_card_text(family),
            mission_checker: Arc::new(move |game: Game| {
                check_disgraced(
                    game.get_queens_table().get_statuses().as_ref().unwrap(),
                    family,
                )
            }),
        }
    }

    fn check_disgraced(statuses: &FamiliesStatuses, family: &CourtierFamily) -> bool {
        // checks if family is disgraced at the court
        statuses.get_family(family).value() == -1
    }

    fn build_check_disgraced_card_text(family: &CourtierFamily) -> String {
        format!("{}s should be disgraced at the court.", family)
    }

    // ################################################################
    // -----------------------count status cards-----------------------
    // ################################################################
    pub fn count_status(statuses: &FamiliesStatuses, checked_status: FamilyStatus) -> u8 {
        // checks how many families have status at the court
        statuses
            .as_array()
            .iter()
            .filter(|status_number| **status_number == checked_status.value())
            .count()
            .try_into()
            .unwrap()
    }

    // ################################################################
    // ----------------------count all piles card----------------------
    // ################################################################
    pub fn count_all_piles(piles_scores: &PilesScores, n: u8) -> bool {
        // checks if each pile contains at least n cards (taking nobles into account)
        for pile_score in piles_scores.as_array() {
            if pile_score < n {
                return false;
            }
        }
        true
    }

    // ################################################################
    // ----------------------count any piles card----------------------
    // ################################################################
    pub fn count_any_piles(piles_scores: &PilesScores, n: u8) -> bool {
        // checks if at least one family contains at least n cards (taking nobles into account)
        for pile_score in piles_scores.as_array() {
            if pile_score >= n {
                return true;
            }
        }
        false
    }

    // TODO: implement the actual missions checks functions
}
