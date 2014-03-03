use sdl2::sdl;
use sdl2::timer;
use sdl2::event;
use sdl2::keycode;

mod graphics;
mod sprite;
mod animated_sprite;
mod input;

pub struct Game {
    display: ~graphics::Graphics,
    sprite: ~animated_sprite::AnimatedSprite,
    input: ~input::Input
}

static FPS: uint = 60;
static TILE_SIZE: i32 = 32;

impl Game {
    pub fn new() {
        sdl::init([sdl::InitEverything]);

        let mut game = Game {
            display: ~graphics::Graphics::new(),
            sprite: ~animated_sprite::AnimatedSprite::new(~"assets/charSprites.bmp", 0, 0, 15, 3),
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

            let current_time = timer::get_ticks();
            self.update(current_time - last_update_time);
            last_update_time = current_time;

            self.draw();

            let elapsed_time = timer::get_ticks() - start_time;
            timer::delay(1000 / FPS - elapsed_time);
        }
    }

    fn update(&mut self, elapsed_time: uint) {
        self.sprite.update(elapsed_time);
    }

    fn draw(&self) {
        self.sprite.draw(self.display, 320, 240);
        self.display.screen.present();
    }
}
