use sdl2::sdl;
use sdl2::timer;
use sdl2::event;
use sdl2::keycode;

mod graphics;
mod player;
mod input;

pub struct Game {
    display: ~graphics::Graphics,
    player: ~player::Player,
    input: ~input::Input
}

static FPS: uint = 60;
static TILE_SIZE: i32 = 32;

impl Game {
    pub fn new() {
        sdl::init([sdl::InitEverything]);

        let mut game = Game {
            display: ~graphics::Graphics::new(),
            player: ~player::Player::new(320,240),
            input: ~input::Input::new()
        };
        game.event_loop();
    }

    fn event_loop(&mut self) {
        let mut last_update_time = timer::get_ticks();

        loop {
            let start_time = timer::get_ticks();
            self.input.begin_new_frame();

            loop {
                match event::poll_event() {
                    event::KeyDownEvent(_,_,key_code,_,_) => {
                        self.input.key_down_event(key_code);
                    },
                    event::KeyUpEvent(_,_,key_code,_,_) => {
                        self.input.key_up_event(key_code);
                    },
                    event::QuitEvent(_) => {
                        return
                    },
                    _ => break
                }
            }
            if self.input.was_pressed(keycode::EscapeKey) {
                return
            }

            if self.input.is_held(keycode::LeftKey) && self.input.is_held(keycode::RightKey) {
                self.player.stop_moving();
            } else if self.input.is_held(keycode::LeftKey) {
                self.player.start_moving_left();
            } else if self.input.is_held(keycode::RightKey) {
                self.player.start_moving_right();
            } else {
                self.player.stop_moving();
            }

            let current_time = timer::get_ticks();
            self.update(current_time - last_update_time);
            last_update_time = current_time;

            self.draw();

            let elapsed_time = timer::get_ticks() - start_time;
            timer::delay(1000 / FPS - elapsed_time);
        }
    }

    fn update(&mut self, elapsed_time: uint) {
        self.player.update(elapsed_time);
    }

    fn draw(&self) {
        self.display.screen.clear();
        self.player.draw(self.display);
        self.display.screen.present();
    }
}
