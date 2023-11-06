//! Comman Line chomp game
//!
//! Ana Macavei 2023
//!
//! TO DO:
//! Try to find a winning move. If there is one, perform it.
//! Otherwise, stall by chomping as little as possible.
//! (You can implement this by chomping the furthest-right piece in
//!  the lowermost nonempty row.)
//!
//! add invalid input for create_board and chomp_effect (lib.rs)
//!
//! create a game over function (lib.rs)
//!
//! Add while loop with the exit condition being the game over function (main)
//!
//! Update the README.md using markdown

use chomp_game::Board;
use prompted::input;
fn main() {
    println!("First, let's create the board!");

    let width: usize = input!("Enter a width: ").trim().parse().unwrap();
    let height: usize = input!("Enter a height: ").trim().parse().unwrap();

    // Testing to see if I can create the board successfully
    let mut board = Board::create_board(width, height);
    Board::display_board(&board);

    // Repeat the board here using while loop AND while loop exits when encountering a
    // (0,0) then it's game over. (create a game over function)
    loop {
        // Ask user to chomp using prompted input method
        let mut col: usize = input!("Enter the column: ").trim().parse().unwrap();
        let mut row: usize = input!("Enter the row: ").trim().parse().unwrap();

        // give out of bounds error if user enters row: <= 3 col: <=4
        if row == 4 && col == 5 {
            println!("Out of bounds, Please enter a row < 3 and a column < 4");
            // have user re-enter input!
            col = input!("Enter the column: ").trim().parse().unwrap();
            row = input!("Enter the row: ").trim().parse().unwrap();
        }

        // if user inputs (0,0) then its game over
        if row == 0 && col == 0 {
            println!("Game Over!");
            break;
        }

        Board::chomp_effect(&mut board, row, col);
        Board::display_board(&board);
        if let Some(winning_move) = Board::negamax(&board) {
            // catch winning move and chomp it
            Board::chomp_effect(&mut board, winning_move.0, winning_move.1);
        }
        Board::display_board(&board);
    }
}
