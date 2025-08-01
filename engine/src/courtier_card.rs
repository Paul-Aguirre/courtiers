use rand::rng;
use rand::seq::SliceRandom;

const TWO_PLAYERS_REMOVED_CARDS: usize = 30;
const THREE_PLAYERS_REMOVED_CARDS: usize = 18;
const FOUR_PLAYERS_REMOVED_CARDS: usize = 6;

#[derive(Clone)]
pub enum CourtierFamily {
    Moth,
    Toad,
    Nightingale,
    Hare,
    Stag,
    Carp,
}

#[derive(Clone, PartialEq)]
pub enum CourtierRole {
    NoRole,   // 4 per family
    Noble,    // 4 per family
    Spy,      // 2 per family
    Assassin, // 2 per family
    Guard,    // 3 per family
}

#[derive(Clone)]
pub struct CourtierCard {
    pub family: CourtierFamily,
    pub role: CourtierRole,
}

pub enum Error {
    TryToKillGuardError(String),
    PlayerNumbreError { player_number: u8 },
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
        _ => Err(Error::PlayerNumbreError { player_number: players_number }),
    }
}
