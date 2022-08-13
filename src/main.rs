use tetris::Game;
use tetris::input::Input;

fn main() {
    let input = Input::new();
    let game = Game::new();
    game.game_loop(&input)
}