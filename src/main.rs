#[crate_id = "cave-story#0.0.1"];

extern crate sdl2;
extern crate collections;

use sdl2::sdl;

mod game;

fn main() {
    game::Game::new();

    sdl::quit();
}
