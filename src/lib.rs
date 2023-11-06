//! Library crate that provides chomp game methods
//!
//! Ana Macavei 2023

//global constants
const MAX_WIDTH: usize = 5;
const MAX_HEIGHT: usize = 4;

#[derive(Clone)]
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
        // if user gives invalid input, simply ask again until they give valid input
        // Initialize the board to true because it would be false if eaten.
        let grid = [[true; MAX_WIDTH]; MAX_HEIGHT];

        Board {
            grid,
            width,
            height,
        }
    }

    // Print a graphical representation of the board after each turn
    pub fn display_board(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.grid[i][j] {
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
    pub fn chomp_effect(&mut self, row: usize, col: usize) {
        //if user gives an invalid input, ask again until they give valid
        //input. give out of bounds error for row: <= 3 col: <=4
        for i in row..self.height {
            for j in col..self.width {
                self.grid[i][j] = false;
            }
        }
    }

    // The negamax algorithm solves any zero-sum perfect-information
    // two-player game (like Chomp). It takes as input a board state and
    // outputs a winning move, if one exists.
    pub fn negamax(&self) -> Option<(usize, usize)> {
        //call self.clone() here
        // for each remaining row r
        for r in 0..self.height {
            // for each remaining
            for c in 0..self.width {
                if r == 0 && c == 0 {
                    continue;
                }
                // Check if r and c is true
                if self.grid[r][c] == true {
                    let mut new_board = self.clone();

                    new_board.chomp_effect(r, c);
                    let possible_move = new_board.negamax();
                    if possible_move == None {
                        return Some((r, c));
                    }
                }
            }
        }
        return None;
    }
}

#[test]
fn test_create_board() {}

fn test_chomp_effect() {}

fn test_negamax() {}
 
