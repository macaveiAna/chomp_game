# chomp_game

This README.md file describes the Rust implementation of the Chomp game.

## Overview

Chomp is a two-player game played on a rectangular board of chocolate squares. Players take turns chomping squares off of the board, removing all squares below and to the right of the chomped square. The player who chomps the square in the upper-left corner of the board loses.

This Rust implementation of Chomp uses a struct called Board to represent the game state. The Board struct contains a two-dimensional array of Boolean values, where each value represents whether or not a square has been eaten. The Board struct also contains fields representing the width and height of the board.

The program also implements a function called negamax(), which uses the negamax algorithm to find a winning move for the current player, if one exists. The negamax algorithm is a recursive algorithm that explores all possible moves and responses, and returns the move that gives the current player the best possible end-of-game result.

## How to Play

To play Chomp, simply run the program and enter the desired board size when prompted. The program will then print the board and ask you to enter a move. To enter a move, simply type the row and column number of the square you want to chomp, separated by a space. For example, to chomp the square in the second row and third column, you would type 2 3.

After you enter a move, the program will perform the move and update the board. The program will then check to see if there is a winning move for the current player. If there is, the program will perform the winning move. Otherwise, the program will stall by chomping as little as possible.

## Writeup - what I did, how it went

I had a lot of difficulty picturing how the chomp function would work. I ended up drawing it out and realized that I would start at the  row and column that the user inputs, and everything to the right of the given square,
and everything below it will become false. I marked the squares that were true with an `X` so that it was clear to me which squares were eaten and which ones were still available.

I ran into this issue when writing the negamax function:
"main has overflowed its stack" for negamax function... I just needed to put in a check to see if the boolean value of the row and column was true.

## Testing

The program includes a number of unit tests to ensure that the game logic is correct. To run the unit tests, simply run the following command:

`cargo test`

If all of the unit tests pass, then you can be confident that the game logic is correct.



## Conclusion

This Rust implementation of the Chomp game is a simple but effective way to learn about the negamax algorithm and other game programming concepts. 
