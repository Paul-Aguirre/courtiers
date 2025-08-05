use std::fmt;

use rand::rng;
use rand::seq::SliceRandom;

const TWO_PLAYERS_REMOVED_CARDS: usize = 30;
const THREE_PLAYERS_REMOVED_CARDS: usize = 18;
const FOUR_PLAYERS_REMOVED_CARDS: usize = 6;

#[derive(Debug)]
pub enum Error {
    TryToKillGuardError(String),
    PlayerNumbreError { player_number: u8 },
}

#[derive(Clone, Debug)]
pub enum CourtierFamily {
    Moth,
    Toad,
    Nightingale,
    Hare,
    Stag,
    Carp,
}

impl CourtierFamily {
    pub fn families_iter() -> std::slice::Iter<'static, CourtierFamily> {
        [
            Self::Moth,
            Self::Toad,
            Self::Nightingale,
            Self::Hare,
            Self::Stag,
            Self::Carp,
        ]
        .iter()
    }
}

impl fmt::Display for CourtierFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, PartialEq, Debug)]
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
    pub fn special_roles_iter() -> std::slice::Iter<'static, CourtierRole> {
        [Self::Noble, Self::Spy, Self::Assassin, Self::Guard].iter()
    }
}

#[derive(Clone)]
pub struct CourtierCard {
    pub family: CourtierFamily,
    pub role: CourtierRole,
}

fn build_family(family: CourtierFamily) -> Vec<CourtierCard> {
    let mut family_deck: Vec<CourtierCard> = Vec::new();

    for _ in 0..=4 {
        family_deck.push(CourtierCard {
            family: family.clone(),
            role: CourtierRole::NoRole,
        });
        family_deck.push(CourtierCard {
            family: family.clone(),
            role: CourtierRole::Noble,
        });
    }

    for _ in 0..=3 {
        family_deck.push(CourtierCard {
            family: family.clone(),
            role: CourtierRole::Guard,
        });
    }

    for _ in 0..=2 {
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
        2 => Ok(deck.drain(..TWO_PLAYERS_REMOVED_CARDS).collect()),
        3 => Ok(deck.drain(..THREE_PLAYERS_REMOVED_CARDS).collect()),
        4 => Ok(deck.drain(..FOUR_PLAYERS_REMOVED_CARDS).collect()),
        5 => Ok(deck),
        _ => Err(Error::PlayerNumbreError {
            player_number: players_number,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_for_courtier_family() {
        assert_eq!(format!("{}", CourtierFamily::Moth), "Moth")
    }

    #[test]
    fn test_display_for_courtier_role() {
        assert_eq!(format!("{}", CourtierRole::Assassin), "Assassin")
    }
}
