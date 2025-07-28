use std::cmp::Ordering;
use std::iter::zip;

use crate::piles::Piles;

const IN_THE_LIGHT: i8 = 1;
const NEUTRAL: i8 = 0;
const DISGRACED: i8 = -1;

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

pub struct QueensTable {
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
    }
}
