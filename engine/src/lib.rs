mod courtier_card;
use courtier_card::CourtierCard;

#[macro_use]
mod piles;
use piles::Piles;

mod queens_table;
use queens_table::QueensTable;

mod mission_card;
use mission_card::MissionCard;

mod player;
use player::Player;

struct Game {
    deck: Vec<CourtierCard>,
    players: Vec<Player>,
    queens_table: QueensTable,
}

impl Game {
    fn init_game(players_number: u8) -> Game {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
