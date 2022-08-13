mod lib;
mod display;
mod input;

fn main() {
    println!("Hello World!");
    let tetris = lib::Tetris::new();

    display::display(tetris.return_grid());
}