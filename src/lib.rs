//! Library crate that provides chomp game methods
//!
//! Ana Macavei 2023


// definition of struct Board using a fixed size
struct Board {
    // vector array of bools
    state: Vec<Vec<bool>>,
    width: usize,
    height: usize,
   
}

// Board type should support the following operations via impl
impl Board {
    
    // Create a board with a given width and height
    pub fn create_board(height: usize, width: usize) {
        // Initialize the board to true because it would be false if eaten.
        let mut state = vec![vec![true; height]; width];

    }

    // Print a graphical representation of the board after each turn
    pub fn display_board() {}

    // Chomp a given square, removing all squares below it and to the right of it
    pub fn chomp_effect() {}

    // This function clones the board
    pub fn clone_board() {}

    // The negamax algorithm solves any zero-sum perfect-information
    // two-player game (like Chomp). It takes as input a board state and
    // outputs a winning move, if one exists.
    pub fn negamax(state: Vec<Vec<bool>>) {
        
    }
}