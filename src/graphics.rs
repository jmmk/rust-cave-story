use collections::HashMap;
use std::rc::Rc;
use sdl2::render;
use sdl2::pixels;
use sdl2::video;
use sdl2::surface;

pub struct Graphics {
    screen: ~render::Renderer,
    sprite_sheets: HashMap<~str, Rc<~surface::Surface>>
}

static WINDOW_HEIGHT: int = 480;
static WINDOW_WIDTH: int = 640;

impl Graphics {
    pub fn new() -> Graphics {
        let display: Graphics;

        match render::Renderer::new_with_window(WINDOW_WIDTH, WINDOW_HEIGHT, [video::InputGrabbed]) {
            Ok(renderer) => {
                display = Graphics {
                    screen: renderer,
                    sprite_sheets: HashMap::<~str, Rc<~surface::Surface>>::new()
                };
            },
            Err(_) => fail!("Could not create Window and Renderer")
        }

        display.screen.set_draw_color(pixels::RGBA(0,0,0,255));
        display.screen.clear();
        display.screen.present();

        return display
    }

    pub fn load_image(&mut self, p: ~str) -> Rc<~surface::Surface> {
        let surface = &*self.sprite_sheets.find_or_insert_with(p, |path| {
            match surface::Surface::from_bmp(&Path::new((*path).clone())) {
                Ok(surface) => Rc::new(~*surface),
                Err(_) => fail!("Could not load bitmap")
            }
        });
        surface.clone()
    }
}
