use game::TILE_SIZE;
use graphics;

use sprite;

pub struct AnimatedSprite {
    sprite: sprite::Sprite,
    frame_time: i32,
    frames: i32,
    current_frame: i32,
    elapsed_time: uint
}

impl AnimatedSprite {
    pub fn new(p: ~str, x: i32, y: i32, fps: i32, frames: i32, display: &mut graphics::Graphics) -> AnimatedSprite {
        AnimatedSprite {
            sprite: sprite::Sprite::new(p, x, y, display),
            frame_time: 1000 / fps,
            frames: frames,
            current_frame: 0,
            elapsed_time: 0
        }
    }
}

impl sprite::HashableSprite for AnimatedSprite {
    fn update(&mut self, elapsed_time: uint) {
        self.elapsed_time += elapsed_time;

        if self.elapsed_time > self.frame_time as uint {
            self.current_frame += 1;
            self.elapsed_time = 0;
            if self.current_frame < self.frames {
                self.sprite.source_rect.x += TILE_SIZE;
            } else {
                self.sprite.source_rect.x -= TILE_SIZE * (self.frames - 1);
                self.current_frame = 0;
            }
        }
    }

    fn draw(&self, display: &graphics::Graphics, x: i32, y: i32) {
        self.sprite.draw(display, x, y);
    }
}
