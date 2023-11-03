//! Library crate that provides chomp game methods
//!
//! Ana Macavei 2023

//global constants
const WIDTH:usize = 5;
const HEIGHT:usize = 4;

// definition of struct Board
pub struct Board {
    // A 2D array of booleans with dimensions 4x5
    grid: [[bool; WIDTH]; HEIGHT],
    width: usize,
    height: usize,
}

// Board type should support the following operations via impl
impl Board {
    pub fn create_board() -> Self {
        // Initialize the board to true because it would be false if eaten.
        let grid = [[true; WIDTH]; HEIGHT];
        let width = WIDTH;
        let height = HEIGHT;

        Self {
            grid,
            width,
            height,
        }
    }

    // Print a graphical representation of the board after each turn
    pub fn display_board(board: &Board) {
        for i in 0..board.height {
            for j in 0..board.width {
                if board.grid[i][j] {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    // Chomp a given square, removing all squares below it and to the right of it
    pub fn chomp_effect() {}

    // This function clones the board
    pub fn clone_board() {}

    // The negamax algorithm solves any zero-sum perfect-information
    // two-player game (like Chomp). It takes as input a board state and
    // outputs a winning move, if one exists.
    pub fn negamax(/*grid: Vec<Vec<bool>>*/) {}
}
