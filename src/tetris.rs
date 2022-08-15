pub mod piece;

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

    // Set the grid at a Pos
    pub fn set_grid(&mut self, pos: piece::Pos, value: i8) {
        if self.get_grid_pos(pos) != value {
            self.grid[pos.0 as usize][pos.1 as usize] = value
        }
    }

    // Check each row of the grid
    // If one is full, remove it and drop
    // the rest of the grid down
    pub fn check_lines() {}

    pub fn get_grid_pos(&self, pos: piece::Pos) -> i8 {
        match self.grid.get(pos.0 as usize) {
            None => return 0,
            _ => return *self.grid.get(pos.0 as usize).unwrap()
            .get(pos.1 as usize).unwrap_or(&0)
        }
    }
}