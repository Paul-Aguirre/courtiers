use std::iter::zip;

use crate::courtier_card::CourtierFamily::{Carp, Hare, Moth, Nightingale, Stag, Toad};
use crate::courtier_card::CourtierRole::{Assassin, Guard, Noble, Spy};
use crate::courtier_card::Error::{self, TryToKillGuard};
use crate::courtier_card::{CourtierCard, CourtierFamily};

type Pile = Vec<CourtierCard>;
// TODO type Pile<T> = Vec<T>;

macro_rules! impl_get_family {
    ($ty:ty, $field_ty:ty) => {
        impl GetFamily<$field_ty> for $ty {
            fn get_family(&self, family: CourtierFamily) -> &$field_ty {
                match family {
                    Moth => &self.moths,
                    Toad => &self.toads,
                    Nightingale => &self.nightingales,
                    Hare => &self.hares,
                    Stag => &self.stags,
                    Carp => &self.carps,
                }
            }
        }
    };
}

pub trait GetFamily<T> {
    fn get_family(&self, family: CourtierFamily) -> &T;
}

#[derive(Clone)]
pub struct PilesScores {
    moths: u8,
    toads: u8,
    nightingales: u8,
    hares: u8,
    stags: u8,
    carps: u8,
}

impl PilesScores {
    fn from_array(a: [u8; 6]) -> Self {
        PilesScores {
            moths: a[0],
            toads: a[1],
            nightingales: a[2],
            hares: a[3],
            stags: a[4],
            carps: a[5],
        }
    }

    pub fn as_array(&self) -> [u8; 6] {
        [
            self.moths,
            self.toads,
            self.nightingales,
            self.hares,
            self.stags,
            self.carps,
        ]
    }
}

impl_get_family!(PilesScores, u8);

pub struct Piles {
    // TODO
    // families: HashMap<Family, Vec[NonSpyRole]>
    pub moths: Pile,
    pub toads: Pile,
    pub nightingales: Pile,
    pub hares: Pile,
    pub stags: Pile,
    pub carps: Pile,
    pub spies: Pile,
}

impl Piles {
    pub fn new() -> Piles {
        Piles {
            moths: Vec::new(),
            toads: Vec::new(),
            nightingales: Vec::new(),
            hares: Vec::new(),
            stags: Vec::new(),
            carps: Vec::new(),
            spies: Vec::new(),
        }
    }
    pub fn add(&mut self, card: CourtierCard) {
        if card.role == Spy {
            self.spies.push(card);
        } else {
            match card.family {
                Moth => self.moths.push(card), // TODO .push(card.role)
                Toad => self.toads.push(card),
                Nightingale => self.nightingales.push(card),
                Hare => self.hares.push(card),
                Stag => self.stags.push(card),
                Carp => self.carps.push(card),
            }
        }
    }

    pub fn remove(&mut self, card: CourtierCard) -> Result<(), Error> {
        // TODO
        // match card.role {
        // Guard =>
        // Spy =>
        // Assassin | NoRole | Noble =>
        //}
        if card.role == Guard {
            Err(TryToKillGuard)
        } else if card.role == Spy {
            todo!();
            // has to deal with a specific card (@ specific index in the pile)
            Ok(())
        } else {
            match card.family {
                Moth => self.moths.remove(
                    self.moths
                        .iter()
                        .position(|pile_card| pile_card.role == card.role)
                        .unwrap(),
                ),
                Toad => self.toads.remove(
                    self.toads
                        .iter()
                        .position(|pile_card| pile_card.role == card.role)
                        .unwrap(),
                ),
                Nightingale => self.nightingales.remove(
                    self.nightingales
                        .iter()
                        .position(|pile_card| pile_card.role == card.role)
                        .unwrap(),
                ),
                Hare => self.hares.remove(
                    self.hares
                        .iter()
                        .position(|pile_card| pile_card.role == card.role)
                        .unwrap(),
                ),
                Stag => self.stags.remove(
                    self.moths
                        .iter()
                        .position(|pile_card| pile_card.role == card.role)
                        .unwrap(),
                ),
                Carp => self.carps.remove(
                    self.carps
                        .iter()
                        .position(|pile_card| pile_card.role == card.role)
                        .unwrap(),
                ),
            };
            Ok(())
        }
    }

    pub fn unpack_spies(&mut self) {
        for _ in 0..self.spies.len() {
            let spy = self.spies.pop().unwrap();
            match spy.family {
                Moth => self.moths.push(spy),
                Toad => self.toads.push(spy),
                Nightingale => self.nightingales.push(spy),
                Hare => self.hares.push(spy),
                Stag => self.stags.push(spy),
                Carp => self.carps.push(spy),
            }
        }
    }

    pub fn as_array(&self) -> [&Vec<CourtierCard>; 6] {
        // does not return the spies attribute
        [
            &self.moths,
            &self.toads,
            &self.nightingales,
            &self.hares,
            &self.stags,
            &self.carps,
        ]
    }

    pub fn tally(&self) -> PilesScores {
        let mut scores: [u8; 6] = [0; 6];
        let piles = self.as_array();

        for (cards, mut score) in zip(piles, scores) {
            for card in cards {
                match card.role {
                    Noble => score += 2,
                    _ => score += 1,
                }
            }
        }

        PilesScores::from_array(scores)
    }
}

impl_get_family!(Piles, Pile);
