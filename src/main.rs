//! Comman Line chomp game
//!
//! Ana Macavei 2023
//!
//! TO DO:
//! Try to find a winning move. If there is one, perform it.
//! Otherwise, stall by chomping as little as possible.
//! (You can implement this by chomping the furthest-right piece in
//! the lowermost nonempty row.) Might need to create a function
//!
//! Update the README.md using markdown
//! 
//! ?Ask nicolas about if user knows that it starts at index 0 or 1
//! ?Do you want AI to go first or user to go first. In this case we just call the negamax function first.
//! 
//! I plan to add if the user wants to play again after game over.

use chomp_game::Board;
use prompted::input;
fn main() {
    println!("First, let's create the board!");

    let mut width: usize = input!("Enter a width: ").trim().parse().unwrap();
    let mut height: usize = input!("Enter a height: ").trim().parse().unwrap();

    // Check for out of bounds when creating the board
    while width > 5 && height > 4 {
        println!("Out of bounds! Enter a board that has a width <= 5 and a height <= 4.");
        width = input!("Enter a width: ").trim().parse().unwrap();
        height = input!("Enter a height: ").trim().parse().unwrap();
    }

    // Once user input is within the correct size constraints, create the board.
    let mut board = Board::create_board(width, height);
    println!(" ");
    Board::display_board(&board);

    loop {
        // Repeat the board here using a loop and have it exit when encountering a
        // (0,0) then it's game over.

        // Ask user to chomp using prompted input! method
        let mut col: usize = input!("Enter the column: ").trim().parse().unwrap();
        let mut row: usize = input!("Enter the row: ").trim().parse().unwrap();

        // Give "out of bounds error" if user enters row: <= 3 col: <=4 or row = 0 col = 0
        // if index starts at 0 then (4,5) will be out of bounds
        while row == 4 && col == 5 {
            println!("Out of bounds! Please enter a row < 3 and a column < 4");
            // have user re-enter input!
            col = input!("Enter the column: ").trim().parse().unwrap();
            row = input!("Enter the row: ").trim().parse().unwrap();
        }
        // if user inputs (0,0) then it's game over
        if row == 0 && col == 0 {
            println!("Game Over!");
            break;
        }

        // add a condition here if the user enters r=0 and c=0 then it's game over
        Board::chomp_effect(&mut board, row, col);
        println!(" ");
        Board::display_board(&board);
        //use "match" because the ai is not moving if there is no winning move
        
       /* 
            if let Some(winning_move) = Board::negamax(&board){
            // catch winning move and chomp it
            Board::chomp_effect(&mut board, winning_move.0, winning_move.1);
        }*/
        match Board::negamax(&board) {
            Some(winning_move) => {
                // Catch winning move and chomp it
                board.chomp_effect(winning_move.0, winning_move.1);
            }
            None => {
                // Force AI to make a move even if it will lose
                // call function that takes the lower most square
                // test a 4x4 board that eats (1,1)
                
                
                
            }
        }
        println!(" ");
        Board::display_board(&board);
    }
}


