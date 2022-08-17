pub mod input;
mod tetris;
mod display;

use crate::tetris::piece;
use std::io::stdin;
use crate::input::Input;

pub struct Game {
    tickrate: u8, // How many times things are checked a second
    maxfps: u8, // Between 1 and 240
    gamespeed: u8, // How quickly r#move(down) is called in ms
}
impl Game {
    pub fn new() -> Game {
        Game{
            tickrate: 20,
            maxfps: 30,
            gamespeed: 250,
        }
    }

    // The actual game loop
    pub fn game_loop(&self, input: &Input) {
        let mut tetris = tetris::Tetris::new();
        let mut piece = piece::Piece::random(piece::Pos(0,0));
        loop {
            tetris.check_lines();
            piece.r#move(piece::Dir::Down, &tetris);

            // Check if piece is dead
            if piece.is_alive() == false {
                piece.apply_to_grid(&mut tetris);
                piece = piece::Piece::random(piece::Pos(0,0));
                // piece = piece::Piece::new(piece::Pieces::Cube, piece::Pos(0,0));
            }
            
            let grid = tetris.return_grid();
            display::display(&grid, &piece);

            let mut input = String::new();

            stdin().read_line(&mut input).expect("Could not read line");
            let input = input.get(0..1).expect("Nothing to get");
            match input {
                "a" => piece.r#move(piece::Dir::Left, &tetris),
                "d" => piece.r#move(piece::Dir::Right, &tetris),
                "e" => piece.rotate(piece::Rotate::Right, &tetris),
                "q" => piece.rotate(piece::Rotate::Left, &tetris),
                " " => break,
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {}
