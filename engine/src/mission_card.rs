pub enum MissionCardColor {
    Blue,
    White,
}

pub struct MissionCard {
    color: MissionCardColor,
    text: String,
}

mod white_cards {
    use std::cmp::Ordering;

    use crate::{
        courtier_card::{CourtierFamily, CourtierRole},
        piles::{GetFamily, Piles, PilesScores},
    };

    fn cmp_neighbor(left: PilesScores, right: PilesScores, family: &CourtierFamily) -> Ordering {
        // compares the number of cards in a specified family between two PliesScores instancess
        left.get_family(family).cmp(right.get_family(family))
    }
    
    fn count_roles_across_families(piles: &Piles, role: &CourtierRole) -> u8 {
        // counts the number of cards with role across all the cards in a Piles instance
        Vec::from(piles.as_array())
            .into_iter()
            .flatten()
            .filter(|card| &card.role == role)
            .count()
            .try_into()
            .unwrap()
        }
    // TODO: implement the actual missions checks functions
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
                return false
            }
        }
        true
    }

    fn count_any_piles(piles_scores: &PilesScores, n: u8) -> bool {
        // checks if at least one family contains at least n cards (taking nobles into account)
        for pile_score in piles_scores.as_array() {
            if pile_score >= n {
                return true
            }
        }
        false
    }

    // TODO: implement the actual missions checks functions
}
