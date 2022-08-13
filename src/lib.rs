pub mod input;
mod tetris;

use crate::input::Input;

pub struct Game {
    tickrate: u8, // Between 
    maxfps: u8, // Between 1 and 240
}
impl Game {
    pub fn new() -> Game {
        Game{
            tickrate: 20,
            maxfps: 30,
        }
    }

    // The actual game loop
    pub fn game_loop(&self, input: &Input) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
