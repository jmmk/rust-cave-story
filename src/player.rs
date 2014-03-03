use game::graphics;
use std::cmp;

mod animated_sprite;

static WALKING_ACCELERATION: f32 = 0.0012;
static MAX_SPEED_X: f32 = 0.325;
static SLOW_DOWN_FACTOR: f32 = 0.8;

pub struct Player {
    sprite: ~animated_sprite::AnimatedSprite,
    x: i32,
    y: i32,
    velocity_x: f32,
    acceleration_x: f32
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        Player {
            sprite: ~animated_sprite::AnimatedSprite::new(
                ~"assets/charSprites.bmp", 0, 0, 15, 3
            ),
            x: x,
            y: y,
            velocity_x: 0.0,
            acceleration_x: 0.0
        }
    }

    pub fn update(&mut self, elapsed_time: uint) {
        self.sprite.update(elapsed_time);

        self.x += (self.velocity_x * elapsed_time as f32).round() as i32;
        self.velocity_x += self.acceleration_x * elapsed_time as f32;

        if self.acceleration_x < 0.0 {
            self.velocity_x = cmp::max(self.velocity_x, -MAX_SPEED_X);
        } else if self.acceleration_x > 0.0 {
            self.velocity_x = cmp::min(self.velocity_x, MAX_SPEED_X);
        } else {
            self.velocity_x *= SLOW_DOWN_FACTOR;
        }
    }

    pub fn draw(&self, display: &graphics::Graphics) {
        self.sprite.draw(display, self.x, self.y);
    }

    pub fn start_moving_left(&mut self) {
        self.acceleration_x = -WALKING_ACCELERATION;
    }

    pub fn start_moving_right(&mut self) {
        self.acceleration_x = WALKING_ACCELERATION;
    }

    pub fn stop_moving(&mut self) {
        self.acceleration_x = 0.0;
    }
}
