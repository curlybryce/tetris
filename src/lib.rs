mod tetris;

use crate::tetris::piece;

use std::time::{Instant, Duration};

// SFML
use sfml::window::{Style, Event, ContextSettings, Key};
use sfml::graphics::{self, *};
use sfml::system::{Vector2f, Vector2u};

pub struct Game {
    tickrate: Duration, // How many times things are checked a second
    maxfps: u64,
    window_geometry: (u32, u32),
    window: RenderWindow,
}
impl Game {
    pub fn new() -> Game {
        let maxfps = 30;
        let geometry = (480, 480);

        // Window setup
        let mut context_settings: ContextSettings = Default::default();
        context_settings.antialiasing_level = 0;

        let mut window = graphics::RenderWindow::new(
            geometry,
            "Test",
            Style::DEFAULT,
            &context_settings);

        window.set_framerate_limit(maxfps);
        window.clear(Color::rgb(250, 250, 250));

        // View
        let view_geometry = (geometry.0 as f32, geometry.1 as f32);
        let view = View::new(
            Vector2f::from(view_geometry) / 2.0,
            Vector2f::from(view_geometry));
        window.set_view(&view);

        Game{
            tickrate: Duration::from_secs(1),
            maxfps: 30,
            window_geometry: geometry,
            window: window,
        }
    }

    pub fn set_geometry(&mut self, geometry: (u32, u32)) {
        self.window_geometry = geometry;
        self.window.set_size(Vector2u::from(geometry));
    }

    fn pause(&mut self) {
        println!("paused");
        loop {
            match self.window.wait_event() {
                Some(Event::GainedFocus) => break,
                 _ => ()
            }
        }
        println!("resumed");
    }

    // The actual game loop
    pub fn game_loop(&mut self) {

        // Textures
        let background = Texture::from_file("assets/background.png").expect("Cannot load texture");
        let bit = Texture::from_file("assets/bit.png").expect("Cannot load texture");

        // Objects
        let mut background = RectangleShape::with_texture(&background);
        background.set_size(Vector2f::new(480.0, 480.0));

        let mut bit = RectangleShape::with_texture(&bit);
        bit.set_size(Vector2f::new(24.0, 24.0));

        // Initialize the background
        self.window.draw(&background);

        // Timing
        let mut tick = Instant::now();
        let mut fpscap = Instant::now();

        // Game setup
        let mut tetris = tetris::Tetris::new();
        let mut piece = piece::Piece::random(piece::Pos(-2,3));
        loop {
            // Process events
            let mut key: Option<Key> = None;
            match self.window.poll_event() {
                Some(n) => match n {
                    Event::Closed => break,
                    Event::Resized {width: w, height: h} => self.set_geometry((w, h)),
                    Event::LostFocus => self.pause(),
                    Event::KeyPressed {code: c, ctrl: _, alt: _, shift: _, system: _} => key = Some(c),
                    Event::KeyReleased {code: _, ctrl: _, alt: _, shift: _, system: _} => key = None,
                    _ => (),
                },
                None => (),
            }

            // Process keys
            use tetris::piece::{Dir, Rotate};
            match key {
                Some(Key::A) => piece.r#move(Dir::Left, &tetris),
                Some(Key::S) => piece.r#move(Dir::Down, &tetris),
                Some(Key::D) => piece.r#move(Dir::Right, &tetris),
                Some(Key::Q) => piece.rotate(Rotate::Left, &tetris),
                Some(Key::E) => piece.rotate(Rotate::Right, &tetris),
                _ => ()
            }

            // Check if piece is dead
            if piece.is_alive() == false {
                piece.apply_to_grid(&mut tetris);
                piece = piece::Piece::random(piece::Pos(-2,3));
                // piece = piece::Piece::new(piece::Pieces::Cube, piece::Pos(0,0));
            }

            // Check if there are full lines
            tetris.check_lines();

            // Execute on tick
            if tick.elapsed() >= self.tickrate {
                // Reset the clock
                tick = Instant::now();

                // Timed logic
                piece.r#move(piece::Dir::Down, &tetris)
            }

            if fpscap.elapsed() >= Duration::from_millis(1 / self.maxfps) {
                // Reset the clock
                fpscap = Instant::now();

                // Draw the background
                self.window.draw(&background);

                // Start drawing everything
                let grid = tetris.return_grid();
    
                // Draw all the bits
                for y in (0..grid.len()).rev() {
                    for x in 0..grid.len() {
                        if *grid.get(y).unwrap_or(&vec![0]).get(x).unwrap_or(&0) == 1 {
                            bit.set_position(((x * 24) as f32, (y * 24) as f32));
                            self.window.draw(&bit); 
                        }
                    }
                }
                for piece_bit in piece.get_bits_pos() {
                    if !(piece_bit.0 < 0 || piece_bit.1 < 0) {
                        bit.set_position(((piece_bit.1 as f32) * 24.0, (piece_bit.0 as f32) * 24.0));
                        self.window.draw(&bit); 
                    }
                }
        
                self.window.display();
            }
        }

        self.window.close();
    }
}

#[cfg(test)]
mod tests {}
