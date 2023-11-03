//! Comman Line chomp game
//!
//! Ana Macavei 2023

// #[derive(Clone)]
use chomp_game::Board;

fn main() {

    let board = Board::create_board(); // testing to see if I can create the board successfully
    Board::display_board(&board);
}
