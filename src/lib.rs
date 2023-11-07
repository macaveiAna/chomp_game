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
        for i in row..self.height {
            for j in col..self.width {
                self.grid[i][j] = false;
            }
        }
    }

    // If there is no winning move, stall by chomping as little as possible.
    // This is implemented by chomping the furthest-right piece in
    // the lowermost nonempty row
    pub fn chomp_stall(&mut self) {
        // use rev() method to start at the rightmost square.
        'outer: for i in (0..self.height).rev() {
            for j in (0..self.width).rev() {
                if self.grid[i][j] {
                    self.grid[i][j] = false;
                    // breaking out of the entire loop structure using 'outer
                    break 'outer;
                }
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
                if self.grid[r][c] {
                    let mut new_board = self.clone();

                    new_board.chomp_effect(r, c);
                    let possible_move = new_board.negamax();
                    if possible_move.is_none() {
                        return Some((r, c));
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_create_board() {
        // test to make sure the board isn't greater than 5x4
        let board = Board::create_board(5, 4);
        assert!(board.width <= MAX_WIDTH);
        assert!(board.height <= MAX_HEIGHT);

        let board_two = Board::create_board(4, 3);
        assert!(board_two.width <= MAX_WIDTH);
        assert!(board_two.height <= MAX_HEIGHT);

        let board_three = Board::create_board(3, 2);
        assert!(board_three.width <= MAX_WIDTH);
        assert!(board_three.height <= MAX_HEIGHT);
    }

    #[test]
    fn test_chomp_effect() {
        let mut board = Board::create_board(5, 4);
        board.chomp_effect(3, 4);
        assert_eq!(board.grid[3][4], false);
    }

    #[test]
    fn test_negamax() {
        // Picking a specific case to test.
        let mut board = Board::create_board(5, 4);
        board.chomp_effect(0, 1);
        let winning_move = board.negamax();
        assert_eq!(winning_move, Some((1, 0)));
    }
}
