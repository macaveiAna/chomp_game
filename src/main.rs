//! Comman Line chomp game
//!
//! Ana Macavei 2023

use chomp_game::Board;
use prompted::input;
fn main() {

    let board = Board::create_board(); // testing to see if I can create the board successfully
    Board::display_board(&board);
    // Ask user to chomp using prompted
    let row = input!("Enter the row: ");
    let col = input!("Enter the column: ");
}
