use std::cmp::Ordering;
use std::iter::zip;

use crate::courtier_card::CourtierFamily;
use crate::courtier_card::CourtierFamily::{Carp, Hare, Moth, Nightingale, Stag, Toad};
use crate::piles::{GetFamily, Piles};

pub type Result<T> = std::result::Result<T, Error>;
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

pub struct QueensTable {
    in_the_light: Piles,
    disgraced: Piles,
    statuses: Option<FamiliesStatuses>,
}

impl QueensTable {
    pub fn new() -> QueensTable {
        QueensTable {
            in_the_light: Piles::new(),
            disgraced: Piles::new(),
            statuses: None,
        }
    }

    pub fn get_in_the_light(&self) -> &Piles {
        &self.in_the_light
    }

    pub fn get_disgraced(&self) -> &Piles {
        &self.disgraced
    }

    pub fn get_statuses(&self) -> &Option<FamiliesStatuses> {
        &self.statuses
    }

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
}
