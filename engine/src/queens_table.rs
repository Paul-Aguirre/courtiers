#![allow(dead_code)]
use std::cmp::Ordering;
use std::iter::zip;

use crate::courtier_card::CourtierFamily;
use crate::courtier_card::CourtierFamily::{Carp, Hare, Moth, Nightingale, Stag, Toad};
use crate::piles::{GetFamily, HiddenSpiesPiles, RevealedSpiesPiles};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum Error {
    FamilyStatusError { number: i8 },
}

pub enum FamilyStatus {
    InTheLight,
    Neutral,
    Disgraced,
}

impl FamilyStatus {
    pub fn value(&self) -> i8 {
        match self {
            FamilyStatus::Disgraced => -1,
            FamilyStatus::Neutral => 0,
            FamilyStatus::InTheLight => 1,
        }
    }

    fn from_number(n: i8) -> Result<FamilyStatus> {
        match n {
            -1 => Ok(FamilyStatus::Disgraced),
            0 => Ok(FamilyStatus::Neutral),
            1 => Ok(FamilyStatus::InTheLight),
            _ => Err(Error::FamilyStatusError { number: n }),
        }
    }
}

pub struct FamiliesStatuses {
    moths: FamilyStatus,
    toads: FamilyStatus,
    nightingales: FamilyStatus,
    hares: FamilyStatus,
    stags: FamilyStatus,
    carps: FamilyStatus,
}

impl FamiliesStatuses {
    fn from_array(a: [i8; 6]) -> Result<Self> {
        Ok(FamiliesStatuses {
            moths: FamilyStatus::from_number(a[0])?,
            toads: FamilyStatus::from_number(a[1])?,
            nightingales: FamilyStatus::from_number(a[2])?,
            hares: FamilyStatus::from_number(a[3])?,
            stags: FamilyStatus::from_number(a[4])?,
            carps: FamilyStatus::from_number(a[5])?,
        })
    }

    pub fn as_array(&self) -> [i8; 6] {
        [
            self.moths.value(),
            self.toads.value(),
            self.nightingales.value(),
            self.hares.value(),
            self.stags.value(),
            self.carps.value(),
        ]
    }
}

impl_get_family!(FamiliesStatuses, FamilyStatus);

pub struct ActiveQueensTable {
    in_the_light: HiddenSpiesPiles,
    disgraced: HiddenSpiesPiles,
}

pub struct GameEndedQueensTable {
    in_the_light: RevealedSpiesPiles,
    disgraced: RevealedSpiesPiles,
}

impl ActiveQueensTable {
    pub fn new() -> ActiveQueensTable {
        ActiveQueensTable {
            in_the_light: HiddenSpiesPiles::new(),
            disgraced: HiddenSpiesPiles::new(),
        }
    }

    pub fn in_the_light(&self) -> &HiddenSpiesPiles {
        &self.in_the_light
    }

    pub fn disgraced(&self) -> &HiddenSpiesPiles {
        &self.disgraced
    }

    pub fn end_game(self) -> GameEndedQueensTable {
        GameEndedQueensTable {
            in_the_light: self.in_the_light.reveal_spies(),
            disgraced: self.disgraced.reveal_spies(),
        }
    }
}

impl GameEndedQueensTable {
    pub fn determine_family_statuses(&self) -> FamiliesStatuses {
        let mut statuses_array: [i8; 6] = [0; 6];
        for (i, (in_the_light_score, disgraced_score)) in zip(
            self.in_the_light.tally().as_array(),
            self.disgraced.tally().as_array(),
        )
        .enumerate()
        {
            statuses_array[i] = match in_the_light_score.cmp(&disgraced_score) {
                Ordering::Equal => FamilyStatus::Neutral.value(),
                Ordering::Greater => FamilyStatus::InTheLight.value(),
                Ordering::Less => FamilyStatus::Disgraced.value(),
            };
        }
        FamiliesStatuses::from_array(statuses_array).unwrap()
    }

    pub fn in_the_light(&self) -> &RevealedSpiesPiles {
        &self.in_the_light
    }

    pub fn disgraced(&self) -> &RevealedSpiesPiles {
        &self.disgraced
    }
}
