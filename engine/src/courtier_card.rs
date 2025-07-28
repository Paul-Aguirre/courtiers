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
    None,     // 4 per family
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
