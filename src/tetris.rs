mod piece;

use piece::Piece;

#[derive(Debug)]
pub struct Tetris {
    grid: Vec<Vec<i8>>
}
impl Tetris {
    pub fn new() -> Tetris {
        Tetris{
            grid: vec!(vec!(0; 10); 20),
        }
    }

    // Return the grid
    pub fn return_grid(&self) -> &Vec<Vec<i8>> {
        &self.grid
    }

    // Set the grid, given a piece
    pub fn set_grid(&mut self, piece: Piece) {}

    // Check each row of the grid
    // If one is full, remove it and drop
    // the rest of the grid down
    pub fn check_lines() {}
}