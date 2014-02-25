extern crate sdl2;

use sdl2::sdl;
use sdl2::timer;
use sdl2::event;
use sdl2::keycode;
use sdl2::video;
use sdl2::render;

pub struct Game;

impl Game {
    pub fn new() -> Game {
        Game
    }

    pub fn start(&self) {
        sdl::init([sdl::InitEverything]);
        let screen = render::Renderer::new_with_window(640, 480, [video::InputGrabbed]).unwrap();

        self.event_loop()
    }

    fn event_loop(&self) {
        loop {
            let start_time = timer::get_ticks();
            loop {
                match event::poll_event() {
                    event::KeyDownEvent(_,_,key_code,_,_) => {
                        println!("{:?}", key_code);
                        if key_code == keycode::EscapeKey {
                            return;
                        }
                    },
                    event::KeyUpEvent(_,_,key_code,_,_) => {
                        println!("{:?}", key_code);
                    },
                    _ => break
                }
            }
            self.update();
            self.draw();

            let elapsed_time = timer::get_ticks() - start_time;
            timer::delay(1000 / 60 - elapsed_time)
        }
    }

    fn update(&self) {

    }

    fn draw(&self) {

    }
}
