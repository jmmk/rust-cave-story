#[crate_id = "cave-story#0.0.1"];
#[feature(default_type_params)];

extern crate sdl2;
extern crate collections;

use sdl2::sdl;

mod game;
mod graphics;
mod player;
mod sprite;
mod input;
mod animated_sprite;

fn main() {
    game::Game::new();

    sdl::quit();
}
