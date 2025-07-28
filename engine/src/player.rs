use crate::Piles;
use crate::MissionCard;
pub struct Player {
    player_name: String,
    domain: Piles,
    mission_cards: (MissionCard, MissionCard)
}

impl Player {
    pub fn compute_score(&self) {}
}