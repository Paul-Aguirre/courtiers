use core::hash;
use std::cell::RefCell;
use std::iter::zip;
use std::rc::Rc;
use std::result;

const IN_THE_LIGHT: i8 = 1;
const NEUTRAL: i8 = 0;
const DISGRACED: i8 = -1;


#[derive(Clone)]
enum CourtierFamily {
    Moth,
    Toad,
    Nightingale,
    Hare,
    Stag,
    Carp,
}

use crate::CourtierFamily::{Carp, Hare, Moth, Nightingale, Stag, Toad};

#[derive(Clone, PartialEq)]
enum CourtierRole {
    None,     // 4 per family
    Noble,    // 4 per family
    Spy,      // 2 per family
    Assassin, // 2 per family
    Guard,    // 3 per family
}

use crate::CourtierRole::{Assassin, Guard, Noble, Spy};

#[derive(Clone)]
struct CourtierCard {
    family: CourtierFamily,
    role: CourtierRole,
}

enum MissionCardColor {
    Blue,
    White,
}

struct MissionCard {
    color: MissionCardColor,
    text: String,
}

struct PilesScores {
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

    fn as_array(&self) -> [u8; 6] {
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

struct Piles {
    moths: Vec<CourtierCard>,
    toads: Vec<CourtierCard>,
    nightingales: Vec<CourtierCard>,
    hares: Vec<CourtierCard>,
    stags: Vec<CourtierCard>,
    carps: Vec<CourtierCard>,
    spies: Vec<CourtierCard>,
}

impl Piles {
    fn add(&mut self, card: CourtierCard) {
        if card.role == Spy {
            self.spies.push(card);
        } else {
            match card.family {
                Moth => self.moths.push(card),
                Toad => self.toads.push(card),
                Nightingale => self.nightingales.push(card),
                Hare => self.hares.push(card),
                Stag => self.stags.push(card),
                Carp => self.carps.push(card),
            }
        }
    }

    fn unpack_spies(&mut self) {
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

    fn as_array_no_spies(&self) -> [&Vec<CourtierCard>; 6] {
        [
            &self.moths,
            &self.toads,
            &self.nightingales,
            &self.hares,
            &self.stags,
            &self.carps,
        ]
    }

    fn tally(&self) -> PilesScores {
        //// let mut moth_score: u8 = 0;
        //// let mut toad_score: u8 = 0;
        //// let mut nightingale_score: u8 = 0;
        //// let mut hare_score: u8 = 0;
        //// let mut stag_score: u8 = 0;
        //// let mut carp_score: u8 = 0;
        //// let scores = [
        ////     (&self.moths, moth_score),
        ////     (&self.toads, toad_score),
        ////     (&self.nightingales, nightingale_score),
        ////     (&self.hares, hare_score),
        ////     (&self.stags, stag_score),
        ////     (&self.carps, carp_score),
        //// ];
        //// for (cards, mut score) in scores {
        ////     for card in cards {
        ////         match card.role {
        ////             CourtierRole::Noble => score += 2,
        ////             _ => score += 1,
        ////         }
        ////     }
        //// }
        //// PilesScores {
        ////     moth: moth_score,
        ////     toad: toad_score,
        ////     nightingale: nightingale_score,
        ////     hare: hare_score,
        ////     stag: stag_score,
        ////     carp: carp_score,
        //// }
        let mut scores: [u8; 6] = [0; 6];
        let piles = self.as_array_no_spies();

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

struct Player {
    player_name: String,
    domain: Piles,
    mission_cards: (MissionCard, MissionCard)
}

impl Player {
    pub fn compute_score(&self) {}
}

struct FamiliesStatues {
    moths: i8,
    toads: i8,
    nightingales: i8,
    hares: i8,
    stags: i8,
    carps: i8,
}

impl FamiliesStatues {
    fn from_array(a: [i8; 6]) -> Self {
        FamiliesStatues {
            moths: a[0],
            toads: a[1],
            nightingales: a[2],
            hares: a[3],
            stags: a[4],
            carps: a[5],
        }
    }

    fn as_array(&self) -> [i8; 6] {
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

use std::cmp::Ordering;

struct QueensTable {
    in_the_light: Piles,
    disgraced: Piles,
}

impl QueensTable {
    pub fn determine_family_statuses(&self) -> FamiliesStatues {
        let mut statuses_array: [i8; 6] = [0; 6];
        for (i, (in_the_light_score, disgraced_score)) in zip(
            self.in_the_light.tally().as_array(),
            self.disgraced.tally().as_array(),
        )
        .enumerate()
        {
            statuses_array[i] = match in_the_light_score.cmp(&disgraced_score) {
                Ordering::Equal => NEUTRAL,
                Ordering::Greater => IN_THE_LIGHT,
                Ordering::Less => DISGRACED,
            };
        }
        FamiliesStatues::from_array(statuses_array)
        // zip(in_the_light_scores, disgraced_scores)
        // .map(|(in_the_light_score, disgraced_score)| {
        //     match in_the_light_score.cmp(&disgraced_score) {
        //         Ordering::Equal => NEUTRAL,
        //         Ordering::Greater => IN_THE_LIGHT,
        //         Ordering::Less => DISGRACED,
        //     }
        // })
        // .collect()
    }
}

fn build_deck(players_number: u8) -> Vec<CourtierCard> {
    todo!()
    // contains logic of what cards can be present
    // shuffles the deck
    // takes out some cards according to the number of players
}

struct Game {
    deck: Vec<CourtierCard>,
    players: Vec<Player>,
    queens_table: QueensTable,
}

#[cfg(test)]
mod tests {}
