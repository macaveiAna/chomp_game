//! Library crate that provides chomp game methods
//!
//! Ana Macavei 2023

//global constants
const MAX_WIDTH: usize = 5;
const MAX_HEIGHT: usize = 4;

// definition of struct Board
pub struct Board {
    // A 2D array of booleans with dimensions 4x5
    grid: [[bool; MAX_WIDTH]; MAX_HEIGHT],
    width: usize,
    height: usize,
}

// Board type should support the following operations via impl
impl Board {
    pub fn create_board(width: usize, height: usize) -> Self {
        // Initialize the board to true because it would be false if eaten.
        
        let grid = [[true; MAX_WIDTH]; MAX_HEIGHT];

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
    // where width represents the rows and height represents the column
    pub fn chomp_effect(board: &mut Board, row: usize, col: usize) {
        // Loop through setting the users x's to false aka eaten
        for i in row..board.height {
            for j in col..board.width {
                board.grid[i][j] = false;
            }
        }
    }

    // The negamax algorithm solves any zero-sum perfect-information
    // two-player game (like Chomp). It takes as input a board state and
    // outputs a winning move, if one exists.
    pub fn negamax(/*grid: Vec<Vec<bool>>*/) {
        //call self.clone() here
    }
}

#[test]
fn test_create_board() {
    
}

fn test_chomp_effect() {
    
}

fn test_negamax() {

}


