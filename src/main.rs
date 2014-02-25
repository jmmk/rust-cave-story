#[crate_id = "cave-story#0.0.1"];
extern crate sdl2;

use sdl2::sdl;

mod game;

fn main() {
    let game = game::Game::new();
    game.start();
    sdl::quit();
}
