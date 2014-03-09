use std::rc::Rc;
use sdl2::surface;
use sdl2::rect;

use game::graphics;
use game::TILE_SIZE;

pub struct Sprite {
    sprite_sheet: Rc<~surface::Surface>,
    source_rect: rect::Rect
}

impl Sprite {
    pub fn new(p: ~str, x: i32, y: i32, display: &mut graphics::Graphics) -> Sprite {
        let source_rect = rect::Rect::new(x, y, TILE_SIZE, TILE_SIZE);

        Sprite {
            sprite_sheet: (*display).load_image(p),
            source_rect: source_rect
        }
    }
}

pub trait HashableSprite {
    fn update(&mut self, elapsed_time: uint) {}
    fn draw(&self, display: &graphics::Graphics, x: i32, y: i32) {}
}

impl HashableSprite for Sprite {
    fn update(&mut self, elapsed_time: uint) {}
    fn draw(&self, display: &graphics::Graphics, x: i32, y: i32) {
        let dest_rect = rect::Rect::new(x, y, TILE_SIZE, TILE_SIZE);

        match display.screen.create_texture_from_surface(*self.sprite_sheet.borrow()) {
            Ok(texture) => {
                display.screen.copy(texture, Some(self.source_rect), Some(dest_rect));
            },
            Err(_) => fail!("Could not create texture from sprite sheet")
        }
    }
}
