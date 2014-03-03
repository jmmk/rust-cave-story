use sdl2::surface;
use sdl2::rect;
use game::graphics;
use game::TILE_SIZE;


pub struct Sprite {
    sprite_sheet: ~surface::Surface,
    source_rect: ~rect::Rect
}

impl Sprite {
    pub fn new(p: ~str, x: i32, y: i32) -> Sprite {
        let sprite: Sprite;
        let source_rect = rect::Rect::new(x, y, TILE_SIZE, TILE_SIZE);

        match surface::Surface::from_bmp(&Path::new(p)) {
            Ok(surface) => {
                sprite = Sprite {
                    sprite_sheet: surface,
                    source_rect: ~source_rect
                }
            },
            Err(_) => fail!("Could not load sprite from bitmap")
        }

        return sprite
    }

    pub fn draw(&self, display: &graphics::Graphics, x: i32, y: i32) {
        let dest_rect = rect::Rect::new(x, y, TILE_SIZE, TILE_SIZE);

        match display.screen.create_texture_from_surface(self.sprite_sheet) {
            Ok(texture) => {
                display.screen.copy(texture, Some(*self.source_rect), Some(dest_rect));
            },
            Err(_) => fail!("Could not create texture from sprite sheet")
        }
    }
}
