//! Comman Line chomp game
//!
//! Ana Macavei 2023

use chomp_game::Board;
use prompted::input;
fn main() {

    println!("First, let's create the board!");

    let width:usize= input!("Enter a width: ").trim().parse().unwrap();
    let height:usize=input!("Enter a height: ").trim().parse().unwrap();
    // Testing to see if I can create the board successfully
    let mut board = Board::create_board(width, height); 
    Board::display_board(&board);
    // Ask user to chomp using prompted input method
    let row: usize = input!("Enter the row: ").trim().parse().unwrap();
    let col: usize = input!("Enter the column: ").trim().parse().unwrap();

    Board::chomp_effect(&mut board, row, col);
    Board::display_board(&board);
    

}
