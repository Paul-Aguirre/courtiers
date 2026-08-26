#![allow(dead_code)]
use std::iter::zip;

use crate::courtier_card::CourtierFamily::{Carp, Hare, Moth, Nightingale, Stag, Toad};
use crate::courtier_card::CourtierRole::{self, Assassin, Guard, NoRole, Noble, Spy};
use crate::courtier_card::Error::{self, TryToKillGuard};
use crate::courtier_card::{CourtierCard, CourtierFamily};

type Pile<T> = Vec<T>;

pub trait GetFamily<T> {
    fn get_family(&self, family: CourtierFamily) -> &T;
}

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

pub trait AddRoleToFamily {
    fn add_role_to_family(&mut self, role: CourtierRole, family: CourtierFamily);
}

macro_rules! impl_add_role_to_family {
    ($ty:ty) => {
        impl AddRoleToFamily for $ty {
            fn add_role_to_family(&mut self, role: CourtierRole, family: CourtierFamily) {
                match family {
                    Moth => self.moths.push(role),
                    Toad => self.toads.push(role),
                    Nightingale => self.nightingales.push(role),
                    Hare => self.hares.push(role),
                    Stag => self.stags.push(role),
                    Carp => self.carps.push(role),
                }
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct HiddenSpiesPiles {
    // TODO
    // families: HashMap<Family, Vec[NonSpyRole]>
    pub moths: Pile<CourtierRole>,
    pub toads: Pile<CourtierRole>,
    pub nightingales: Pile<CourtierRole>,
    pub hares: Pile<CourtierRole>,
    pub stags: Pile<CourtierRole>,
    pub carps: Pile<CourtierRole>,
    pub spies: Pile<CourtierFamily>,
}

impl_add_role_to_family!(HiddenSpiesPiles);
impl_get_family!(HiddenSpiesPiles, Pile<CourtierRole>);

#[derive(Debug)]
pub struct RevealedSpiesPiles {
    pub moths: Pile<CourtierRole>,
    pub toads: Pile<CourtierRole>,
    pub nightingales: Pile<CourtierRole>,
    pub hares: Pile<CourtierRole>,
    pub stags: Pile<CourtierRole>,
    pub carps: Pile<CourtierRole>,
}

impl_add_role_to_family!(RevealedSpiesPiles);

impl HiddenSpiesPiles {
    pub fn new() -> HiddenSpiesPiles {
        HiddenSpiesPiles {
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
            self.spies.push(card.family);
        } else {
            self.add_role_to_family(card.role, card.family);
        }
    }

    pub fn remove(&mut self, card: CourtierCard) -> Result<(), Error> {
        match card.role {
            Guard => Err(TryToKillGuard),
            Spy => {
                // TODO
                todo!();
            } // has to deal with a specific card (@ specific index in the pile)
            Assassin | NoRole | Noble => {
                match card.family {
                    Moth => self.moths.remove(
                        self.moths
                            .iter()
                            .position(|role| *role == card.role)
                            .unwrap(),
                    ),
                    Toad => self.toads.remove(
                        self.toads
                            .iter()
                            .position(|role| *role == card.role)
                            .unwrap(),
                    ),
                    Nightingale => self.nightingales.remove(
                        self.nightingales
                            .iter()
                            .position(|role| *role == card.role)
                            .unwrap(),
                    ),
                    Hare => self.hares.remove(
                        self.hares
                            .iter()
                            .position(|role| *role == card.role)
                            .unwrap(),
                    ),
                    Stag => self.stags.remove(
                        self.moths
                            .iter()
                            .position(|role| *role == card.role)
                            .unwrap(),
                    ),
                    Carp => self.carps.remove(
                        self.carps
                            .iter()
                            .position(|role| *role == card.role)
                            .unwrap(),
                    ),
                };
                Ok(())
            }
        }
    }

    pub fn reveal_spies(mut self) -> RevealedSpiesPiles {
        let mut unpacked_spies_piles = RevealedSpiesPiles::new();
        for _ in 0..self.spies.len() {
            unpacked_spies_piles.add_role_to_family(Spy, self.spies.pop().unwrap());
        }
        unpacked_spies_piles
    }
}

impl RevealedSpiesPiles {
    fn new() -> Self {
        RevealedSpiesPiles {
            moths: Vec::new(),
            toads: Vec::new(),
            nightingales: Vec::new(),
            hares: Vec::new(),
            stags: Vec::new(),
            carps: Vec::new(),
        }
    }

    pub fn as_array(&self) -> [&Vec<CourtierRole>; 6] {
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
        let scores: [u8; 6] = [0; 6];
        let piles = self.as_array();

        for (roles, mut score) in zip(piles, scores) {
            for role in roles {
                match role {
                    Noble => score += 2,
                    _ => score += 1,
                }
            }
        }

        PilesScores::from_array(scores)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piles_scores_from_array() {
        let scores_array: [u8; 6] = [1, 2, 3, 4, 5, 6];
        let scores_piles = PilesScores::from_array(scores_array);
        assert_eq!(
            scores_piles,
            PilesScores {
                moths: 1,
                toads: 2,
                nightingales: 3,
                hares: 4,
                stags: 5,
                carps: 6
            }
        )
    }

    #[test]
    fn test_piles_scores_as_array() {
        let scores_piles = PilesScores {
            moths: 1,
            toads: 2,
            nightingales: 3,
            hares: 4,
            stags: 5,
            carps: 6,
        };
        let scores_array: [u8; 6] = [1, 2, 3, 4, 5, 6];
        assert_eq!(scores_piles.as_array(), scores_array)
    }

    #[test]
    fn test_piles_scores_get_family() {
        let scores_piles = PilesScores {
            moths: 1,
            toads: 2,
            nightingales: 3,
            hares: 4,
            stags: 5,
            carps: 6,
        };
        for (i, family) in zip(1..=6, CourtierFamily::families_iter()) {
            assert_eq!(*scores_piles.get_family(family), i)
        }
    }

    #[test]
    fn test_piles_add_role_to_family() {
        let mut piles = HiddenSpiesPiles::new();
        piles.add_role_to_family(CourtierRole::NoRole, CourtierFamily::Moth);
        piles.add_role_to_family(CourtierRole::Assassin, CourtierFamily::Moth);
        piles.add_role_to_family(CourtierRole::Noble, CourtierFamily::Carp);
        piles.add_role_to_family(CourtierRole::Guard, CourtierFamily::Stag);

        assert_eq!(
            piles,
            HiddenSpiesPiles {
                moths: vec![CourtierRole::NoRole, CourtierRole::Assassin],
                toads: Vec::new(),
                nightingales: Vec::new(),
                hares: Vec::new(),
                stags: vec![CourtierRole::Guard],
                carps: vec![CourtierRole::Noble],
                spies: Vec::new(),
            }
        )
    }

    #[test]
    fn test_piles_add_spy() {
        let mut piles = HiddenSpiesPiles::new();
        piles.add(CourtierCard {
            family: CourtierFamily::Nightingale,
            role: CourtierRole::Spy,
        });
        piles.add(CourtierCard {
            family: CourtierFamily::Toad,
            role: CourtierRole::Spy,
        });

        assert_eq!(
            piles,
            HiddenSpiesPiles {
                moths: Vec::new(),
                toads: Vec::new(),
                nightingales: Vec::new(),
                hares: Vec::new(),
                stags: Vec::new(),
                carps: Vec::new(),
                spies: vec![CourtierFamily::Nightingale, CourtierFamily::Toad],
            }
        )
    }

    #[test]
    fn test_piles_add_no_spy_role() {
        let mut piles = HiddenSpiesPiles::new();
        piles.add(CourtierCard {
            family: CourtierFamily::Nightingale,
            role: CourtierRole::Guard,
        });
        piles.add(CourtierCard {
            family: CourtierFamily::Toad,
            role: CourtierRole::Assassin,
        });
        piles.add(CourtierCard {
            family: CourtierFamily::Toad,
            role: CourtierRole::NoRole,
        });
        piles.add(CourtierCard {
            family: CourtierFamily::Hare,
            role: CourtierRole::Noble,
        });
        assert_eq!(
            piles,
            HiddenSpiesPiles {
                moths: Vec::new(),
                toads: vec![CourtierRole::Assassin, CourtierRole::NoRole],
                nightingales: vec![CourtierRole::Guard],
                hares: vec![CourtierRole::Noble],
                stags: Vec::new(),
                carps: Vec::new(),
                spies: Vec::new(),
            }
        )
    }

    #[test]
    #[ignore = "Not yet implemented"]
    fn test_piles_remove() {
        todo!()
    }

    #[test]
    fn test_piles_reveal_spies() {
        todo!()
    }

    #[test]
    fn test_piles_as_array() {
        todo!()
    }

    #[test]
    fn test_piles_tally() {
        todo!()
    }

    #[test]
    fn test_piles_get_family() {
        todo!()
    }
}
