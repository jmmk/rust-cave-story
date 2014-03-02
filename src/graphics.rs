use sdl2::render;
use sdl2::pixels;
use sdl2::video;

pub struct Graphics {
    screen: ~render::Renderer
}

static WINDOW_HEIGHT: int = 480;
static WINDOW_WIDTH: int = 640;

impl Graphics {
    pub fn new() -> Graphics {
        let display: Graphics;

        match render::Renderer::new_with_window(WINDOW_WIDTH, WINDOW_HEIGHT, [video::InputGrabbed]) {
            Ok(renderer) => {
                display = Graphics {
                    screen: renderer
                };
            },
            Err(_) => fail!("Could not create Window and Renderer")
        }

        display.screen.set_draw_color(pixels::RGBA(0,0,0,255));
        display.screen.clear();
        display.screen.present();

        return display
    }
}
