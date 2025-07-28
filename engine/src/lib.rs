mod courtier_card;
use courtier_card::CourtierCard;

mod piles;
use piles::Piles;

mod queens_table;
use queens_table::QueensTable;

mod mission_card;
use mission_card::MissionCard;

mod player;
use player::Player;

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
