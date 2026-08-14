#![allow(dead_code)]
use std::fmt;

use rand::seq::SliceRandom;
use thiserror::Error;

const TWO_PLAYERS_REMOVED_CARDS: usize = 30;
const THREE_PLAYERS_REMOVED_CARDS: usize = 18;
const FOUR_PLAYERS_REMOVED_CARDS: usize = 6;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Guards cannot be killed.")]
    TryToKillGuard,
    #[error("This game accepts from 2 to 5 players only. Got: {number_of_players:?}")]
    WrongNumberOfPlayers { number_of_players: u8 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CourtierFamily {
    Moth,
    Toad,
    Nightingale,
    Hare,
    Stag,
    Carp,
}

impl CourtierFamily {
    pub fn families_iter() -> impl Iterator<Item = CourtierFamily> {
        [
            Self::Moth,
            Self::Toad,
            Self::Nightingale,
            Self::Hare,
            Self::Stag,
            Self::Carp,
        ]
        .into_iter()
    }
}

// TODO delete that impl Display??
impl fmt::Display for CourtierFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CourtierRole {
    NoRole,   // 4 per family
    Noble,    // 4 per family
    Spy,      // 2 per family
    Assassin, // 2 per family
    Guard,    // 3 per family
}

impl fmt::Display for CourtierRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl CourtierRole {
    // TODO impl Iterator<Item = CourtierRole> (not yet necessary)
    pub fn special_roles_iter() -> impl Iterator<Item = CourtierRole> {
        [Self::Noble, Self::Spy, Self::Assassin, Self::Guard].into_iter()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CourtierCard {
    pub family: CourtierFamily,
    pub role: CourtierRole,
}

fn build_family(family: CourtierFamily) -> Vec<CourtierCard> {
    let mut family_deck: Vec<CourtierCard> = Vec::new();

    for _ in 0..4 {
        family_deck.push(CourtierCard {
            family: family.clone(),
            role: CourtierRole::NoRole,
        });
        family_deck.push(CourtierCard {
            family: family.clone(),
            role: CourtierRole::Noble,
        });
    }

    for _ in 0..3 {
        family_deck.push(CourtierCard {
            family: family.clone(),
            role: CourtierRole::Guard,
        });
    }

    for _ in 0..2 {
        family_deck.push(CourtierCard {
            family: family.clone(),
            role: CourtierRole::Assassin,
        });
        family_deck.push(CourtierCard {
            family: family.clone(),
            role: CourtierRole::Spy,
        });
    }

    family_deck
}

pub fn build_deck(players_number: u8) -> Result<Vec<CourtierCard>, Error> {
    let mut rng = rand::rng();
    let mut deck: Vec<CourtierCard> = Vec::new();

    deck.append(&mut build_family(CourtierFamily::Moth));
    deck.append(&mut build_family(CourtierFamily::Toad));
    deck.append(&mut build_family(CourtierFamily::Nightingale));
    deck.append(&mut build_family(CourtierFamily::Hare));
    deck.append(&mut build_family(CourtierFamily::Stag));
    deck.append(&mut build_family(CourtierFamily::Carp));

    deck.shuffle(&mut rng);

    match players_number {
        2 => {
            let _: Vec<CourtierCard> = deck.drain(..TWO_PLAYERS_REMOVED_CARDS).collect();
            Ok(deck)
        }
        3 => {
            let _: Vec<CourtierCard> = deck.drain(..THREE_PLAYERS_REMOVED_CARDS).collect();
            Ok(deck)
        }
        4 => {
            let _: Vec<CourtierCard> = deck.drain(..FOUR_PLAYERS_REMOVED_CARDS).collect();
            Ok(deck)
        }
        5 => Ok(deck),
        _ => Err(Error::WrongNumberOfPlayers {
            number_of_players: players_number,
        }),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_families_iter() {
        let mut fams_iter = CourtierFamily::families_iter();
        assert_eq!(fams_iter.next(), Some(CourtierFamily::Moth));
        assert_eq!(fams_iter.next(), Some(CourtierFamily::Toad));
        assert_eq!(fams_iter.next(), Some(CourtierFamily::Nightingale));
        assert_eq!(fams_iter.next(), Some(CourtierFamily::Hare));
        assert_eq!(fams_iter.next(), Some(CourtierFamily::Stag));
        assert_eq!(fams_iter.next(), Some(CourtierFamily::Carp));
        assert_eq!(fams_iter.next(), None);
    }

    #[test]
    fn test_display_for_courtier_family() {
        assert_eq!(format!("{}", CourtierFamily::Moth), "Moth")
    }

    #[test]
    fn test_display_for_courtier_role() {
        assert_eq!(format!("{}", CourtierRole::Assassin), "Assassin")
    }

    #[test]
    fn test_special_roles_iter() {
        let mut spec_roles_iter = CourtierRole::special_roles_iter();
        assert_eq!(spec_roles_iter.next(), Some(CourtierRole::Noble));
        assert_eq!(spec_roles_iter.next(), Some(CourtierRole::Spy));
        assert_eq!(spec_roles_iter.next(), Some(CourtierRole::Assassin));
        assert_eq!(spec_roles_iter.next(), Some(CourtierRole::Guard));
        assert_eq!(spec_roles_iter.next(), None);
    }

    #[test]
    fn test_build_family() {
        let moth_family = build_family(CourtierFamily::Moth);
        assert_eq!(moth_family.len(), 15);

        let mut counts = HashMap::new();
        for card in moth_family {
            assert_eq!(card.family, CourtierFamily::Moth);
            assert_ne!(card.family, CourtierFamily::Toad);
            *counts.entry(card.role.to_string()).or_insert(0) += 1;
        }

        let mut goal_counts = HashMap::new();
        goal_counts.insert(CourtierRole::NoRole.to_string(), 4u8);
        goal_counts.insert(CourtierRole::Noble.to_string(), 4u8);
        goal_counts.insert(CourtierRole::Spy.to_string(), 2u8);
        goal_counts.insert(CourtierRole::Assassin.to_string(), 2u8);
        goal_counts.insert(CourtierRole::Guard.to_string(), 3u8);
        assert_eq!(goal_counts, counts)
    }

    // Test build_deck
    #[test]
    fn test_build_deck_have_correct_lengths() {
        {
            let deck_2p = build_deck(2);
            assert_eq!(deck_2p.unwrap().len(), 60);
        }
        {
            let deck_3p = build_deck(3);
            assert_eq!(deck_3p.unwrap().len(), 72);
        }
        {
            let deck_4p = build_deck(4);
            assert_eq!(deck_4p.unwrap().len(), 84);
        }
        {
            let deck_5p = build_deck(5);
            assert_eq!(deck_5p.unwrap().len(), 90);
        }
    }

    #[test]
    fn test_build_deck_wrong_number_of_players() {
        let wrong_deck = build_deck(42);
        // assert!(wrong_deck.is_err());
        assert!(wrong_deck.is_err_and(|x| x
            == Error::WrongNumberOfPlayers {
                number_of_players: 42
            }))
    }

    #[test]
    fn test_build_deck_has_correct_families_and_roles() {
        //! Can only be tested simply on a 5 players deck
        let deck = build_deck(5).unwrap();
        let mut role_counts = HashMap::new();
        let mut family_counts = HashMap::new();

        for card in deck {
            *role_counts.entry(card.role.to_string()).or_insert(0) += 1;
            *family_counts.entry(card.family.to_string()).or_insert(0) += 1;
        }

        let mut role_counts_goal = HashMap::new();

        role_counts_goal.insert(CourtierRole::NoRole.to_string(), 24u8);
        role_counts_goal.insert(CourtierRole::Noble.to_string(), 24u8);
        role_counts_goal.insert(CourtierRole::Spy.to_string(), 12u8);
        role_counts_goal.insert(CourtierRole::Assassin.to_string(), 12u8);
        role_counts_goal.insert(CourtierRole::Guard.to_string(), 18u8);

        assert_eq!(role_counts, role_counts_goal);

        for family in CourtierFamily::families_iter() {
            assert_eq!(*family_counts.get(&family.to_string()).unwrap(), 15)
        }
    }

    #[test]
    fn test_build_deck_is_shuffled() {
        //! Can only be tested simply on a 5 players deck
        let shuffled_deck = build_deck(5).unwrap();
        let mut sorted_deck: Vec<CourtierCard> = Vec::new();

        sorted_deck.append(&mut build_family(CourtierFamily::Moth));
        sorted_deck.append(&mut build_family(CourtierFamily::Toad));
        sorted_deck.append(&mut build_family(CourtierFamily::Nightingale));
        sorted_deck.append(&mut build_family(CourtierFamily::Hare));
        sorted_deck.append(&mut build_family(CourtierFamily::Stag));
        sorted_deck.append(&mut build_family(CourtierFamily::Carp));

        assert_ne!(shuffled_deck, sorted_deck)
    }
}
