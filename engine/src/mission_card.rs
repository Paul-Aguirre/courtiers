use std::cmp::Ordering;
use std::sync::Arc;

use crate::{
    courtier_card::{CourtierFamily, CourtierRole},
    game::Game,
    mission_card::white_cards::{build_cmp_neighbor_card, build_count_role_card},
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

        [cmp_cards, count_role_cards].concat()
    }

    pub fn build_blue_deck() -> Vec<MissionCard> {
        todo!()
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
    use crate::{
        courtier_card::CourtierFamily,
        piles::{GetFamily, PilesScores},
        queens_table::{FamiliesStatuses, FamilyStatus},
    };

    // blue cards
    fn check_status(statuses: FamiliesStatuses, family: &CourtierFamily) -> bool {
        // checks if family is disgraced at the court
        statuses.get_family(family).value() == -1
    }

    fn count_status(statuses: FamiliesStatuses, checked_status: FamilyStatus) -> u8 {
        // checks how many families have status at the court
        statuses
            .as_array()
            .iter()
            .filter(|status_number| **status_number == checked_status.value())
            .count()
            .try_into()
            .unwrap()
    }

    fn count_all_piles(piles_scores: &PilesScores, n: u8) -> bool {
        // checks if each pile contains at least n cards (taking nobles into account)
        for pile_score in piles_scores.as_array() {
            if pile_score < n {
                return false;
            }
        }
        true
    }

    fn count_any_piles(piles_scores: &PilesScores, n: u8) -> bool {
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
