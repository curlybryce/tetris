pub mod input;
mod tetris;
mod display;

use crate::tetris::piece;
use std::io::stdin;
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
    pub fn game_loop(&self, input: &Input) {
        let mut tetris = tetris::Tetris::new();
        let mut piece = piece::Piece::random();
        loop {
            piece.r#move(piece::Dir::Down, &tetris);

            // Check if piece is dead
            if piece.is_alive() == false {
                piece.apply_to_grid(&mut tetris);
                // piece = piece::Piece::random();
                piece = piece::Piece::new(piece::Pieces::Cube);
            }
            
            let grid = tetris.return_grid();
            display::display(&grid, &piece);

            let mut input = String::new();

            stdin().read_line(&mut input).expect("Could not read line");
            let input = input.get(0..1).expect("Nothing to get");
            match input {
                "a" => piece.r#move(piece::Dir::Left, &tetris),
                "d" => piece.r#move(piece::Dir::Right, &tetris),
                "e" => piece.rotate(piece::Rotate::Right),
                "q" => piece.rotate(piece::Rotate::Right),
                " " => break,
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {}
