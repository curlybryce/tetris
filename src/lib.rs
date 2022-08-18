mod tetris;

use crate::tetris::piece;

use std::time::{Instant, Duration};
use rand::Rng;

// SFML
use sfml::window::{Style, Event, ContextSettings, Key};
use sfml::graphics::{self, *};
use sfml::system::{Vector2f, Vector2u};

pub struct Game {
    tickrate: Duration, // How many times things are checked a second
    low_tickrate: Duration, // What's the lowest speed the game can run at
    maxfps: u64,
    window_geometry: (u32, u32),
    window: RenderWindow,
    score: u64,
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
        window.set_key_repeat_enabled(false);
        window.clear(Color::rgb(250, 250, 250));

        // View
        let view_geometry = (geometry.0 as f32, geometry.1 as f32);
        let mut view = View::new(
            Vector2f::from(view_geometry) / 2.0,
            Vector2f::from(view_geometry));
            view.set_viewport(&FloatRect{left: 0.0, top: 0.0, width: 1.0, height: 1.0});
        window.set_view(&view);

        Game{
            tickrate: Duration::from_millis(1000),
            low_tickrate: Duration::from_millis(150),
            maxfps: 30,
            window_geometry: geometry,
            window: window,
            score: 0,
        }
    }

    pub fn get_score(&self) -> u64 {
        self.score
    }
    pub fn set_score(&mut self, value: u64) {
        if value > u64::MAX {
            self.score = u64::MAX
        } else {
            self.score = value;
        }
    }

    pub fn get_tickrate(&self) -> Duration {
        self.tickrate
    }
    pub fn set_tickrate(&mut self, duration: Duration) {
        if duration < self.low_tickrate {
            self.tickrate = self.low_tickrate
        } else {
            self.tickrate = duration
        }
    }

    pub fn set_geometry(&mut self, geometry: (u32, u32)) {
        self.window_geometry = geometry;
        self.window.set_size(Vector2u::from(geometry));
    }

    fn pause(&mut self) {
        loop {
            match self.window.wait_event() {
                Some(Event::GainedFocus) => break,
                 _ => ()
            }
        }
    }

    // The actual game loop
    pub fn game_loop(&mut self) {
        // Textures
        // Background
        let mut background = Texture::from_file("assets/background.png").expect("Cannot load texture");
        background.set_smooth(false);

        // Bits
        let mut grey = Texture::from_file("assets/dead.png").expect("Cannot load texture");
        grey.set_smooth(false);
        let mut red = Texture::from_file("assets/red.png").expect("Cannot load texture");
        red.set_smooth(false);
        let mut green = Texture::from_file("assets/green.png").expect("Cannot load texture");
        green.set_smooth(false);
        let mut blue = Texture::from_file("assets/blue.png").expect("Cannot load texture");
        blue.set_smooth(false);

        let bits_list = [&red, &green, &blue];


        // Objects
        let mut background = RectangleShape::with_texture(&background);
        background.set_size(Vector2f::new(480.0, 480.0));

        let mut bit = RectangleShape::with_texture(&blue);
        bit.set_size(Vector2f::new(24.0, 24.0));

        // Initialize the background
        self.window.draw(&background);

        // Timing
        let mut tick = Instant::now();
        let mut fpscap = Instant::now();
        let mut key_count = 0;

        // Game setup
        let mut tetris = tetris::Tetris::new();
        let mut piece = piece::Piece::random(piece::Pos(-2,3));
        let mut next_piece = piece::Piece::random(piece::Pos(3,13));
        let mut key = None;
        'main: loop {
            // Clear everything from display
            self.window.clear(Color::rgb(0,0,0));

            // Process events
            for x in self.window.poll_event() {
                match x {
                    Event::Closed => break 'main,
                    Event::Resized {width: w, height: h} => self.set_geometry((w, h)),
                    Event::LostFocus => self.pause(),
                    Event::KeyPressed {code: c, ctrl: _, alt: _, shift: _, system: _} => key = Some(c),
                    Event::KeyReleased {code: _, ctrl: _, alt: _, shift: _, system: _} => key = None,
                    _ => (),
                };
            }

            // Check if piece is dead
            if piece.is_alive() == false {
                if piece.apply_to_grid(&mut tetris) == false {
                    // Game has been lost
                    break
                }
                piece = next_piece;
                piece.set_pos(tetris::piece::Pos(-2, 3));
                next_piece = piece::Piece::random(piece::Pos(3,13));

                // Set the color
                let x = match rand::thread_rng().gen_range(0..bits_list.len()) {
                    n => bits_list[n]
                };
                bit.set_texture(&x, false);
            }
            
            // Check if there are full lines
            match tetris.check_lines() {
                0 => (),
                n => {self.set_score(self.get_score() + (n * n) as u64);
                    self.set_tickrate(self.get_tickrate() - Duration::from_millis(n as u64 * 25));},
            }

            // Execute on tick
            if tick.elapsed() >= self.get_tickrate() {
                // Reset the clock
                tick = Instant::now();
                
                // Timed logic
                piece.r#move(piece::Dir::Down, &tetris)
            }
            
            
            
            if fpscap.elapsed() >= Duration::from_millis(1000 / self.maxfps) {
                // Reset the clock
                fpscap = Instant::now();
                
                
                // Process keys
                // Limited keys
                if key_count == 0 {
                    match key {
                        Some(Key::A) => {piece.r#move(tetris::piece::Dir::Left, &tetris)},
                        Some(Key::D) => {piece.r#move(tetris::piece::Dir::Right, &tetris)},
                        _ => ()
                    }
                }
                //Process keys
                // Unlimited keys
                match key {
                    Some(Key::S) => piece.r#move(tetris::piece::Dir::Down, &tetris),
                    Some(Key::Q) => {piece.rotate(tetris::piece::Rotate::Left, &tetris); key = None},
                    Some(Key::E) => {piece.rotate(tetris::piece::Rotate::Right, &tetris); key = None},
                    Some(Key::Escape) => break,
                    _ => ()
                }
                // Make the limit feel good
                if key != None {
                    key_count += 1;
                } else {
                    key_count = 0;
                }
                if key_count > 2 {
                    key_count = 0;
                }

                // Draw the background
                self.window.draw(&background);

                // Start drawing everything
                let grid = tetris.return_grid();
    

                // Draw all the bits
                // Set the color
                let old_texture = bit.texture().unwrap();
                bit.set_texture(&grey, false);
                for y in (0..grid.len()).rev() {
                    for x in 0..grid.len() {
                        if *grid.get(y).unwrap_or(&vec![0]).get(x).unwrap_or(&0) == 1 {
                            bit.set_position(((x * 24) as f32, (y * 24) as f32));
                            self.window.draw(&bit); 
                        }
                    }
                }
                bit.set_texture(&old_texture, false);

                for piece_bit in piece.get_bits_pos() {
                    if !(piece_bit.0 < 0 || piece_bit.1 < 0) {
                        bit.set_position(((piece_bit.1 as f32) * 24.0, (piece_bit.0 as f32) * 24.0));
                        self.window.draw(&bit); 
                    }
                }
                for piece_bit in next_piece.get_bits_pos() {
                    if !(piece_bit.0 < 0 || piece_bit.1 < 0) {
                        bit.set_position(((piece_bit.1 as f32) * 24.0, (piece_bit.0 as f32) * 24.0));
                        self.window.draw(&bit); 
                    }
                }
        
                self.window.display();
            }
        }

        println!("{}", self.get_score());
        self.window.close();
    }
}