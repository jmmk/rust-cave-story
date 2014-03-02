use sdl2::sdl;
use sdl2::timer;
use sdl2::event;
use sdl2::keycode;

mod graphics;
mod sprite;

pub struct Game {
    display: ~graphics::Graphics,
    sprite: ~sprite::Sprite
}

static FPS: uint = 60;

impl Game {
    pub fn new() {
        sdl::init([sdl::InitEverything]);

        let game = Game {
            display: ~graphics::Graphics::new(),
            sprite: ~sprite::Sprite::new(~"assets/charSprites.bmp", 0, 0)
        };
        game.event_loop();
    }

    fn event_loop(&self) {
        loop {
            let start_time = timer::get_ticks();

            loop {
                match event::poll_event() {
                    event::KeyDownEvent(_,_,key_code,_,_) => {
                        if key_code == keycode::EscapeKey {
                            return
                        }
                    },
                    event::KeyUpEvent(_,_,key_code,_,_) => {
                        if key_code == keycode::EscapeKey {
                            return
                        }
                    },
                    event::QuitEvent(_) => {
                        return
                    },
                    _ => break
                }
            }

            self.update();
            self.draw();

            let elapsed_time = timer::get_ticks() - start_time;
            timer::delay(1000 / FPS - elapsed_time);
        }
    }

    fn update(&self) {

    }

    fn draw(&self) {
        self.sprite.draw(self.display, 320, 240);
        self.display.screen.present();
    }
}
