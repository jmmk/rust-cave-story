use std::cmp;
use collections::HashMap;
use std::hash::Hash;

use game::TILE_SIZE;
use animated_sprite;
use graphics;
use sprite;

static WALKING_ACCELERATION: f32 = 0.0012;
static MAX_SPEED_X: f32 = 0.325;
static SLOW_DOWN_FACTOR: f32 = 0.8;

#[deriving(Hash)]
#[deriving(Eq)]
enum MotionType {
    Standing,
    Walking
}

#[deriving(Hash)]
#[deriving(Eq)]
enum HorizontalFacing {
    Left,
    Right
}

#[deriving(Hash)]
#[deriving(Eq)]
pub struct SpriteState {
    motion_type: MotionType,
    horizontal_facing: HorizontalFacing
}

impl SpriteState {
    pub fn new(motion: MotionType, facing: HorizontalFacing) -> SpriteState {
        SpriteState {
            motion_type: motion,
            horizontal_facing: facing
        }
    }
}

pub struct Player {
    sprites: HashMap<SpriteState, ~sprite::HashableSprite:>,
    x: i32,
    y: i32,
    velocity_x: f32,
    acceleration_x: f32,
    horizontal_facing: HorizontalFacing
}

impl Player {
    pub fn new(x: i32, y: i32, display: &mut graphics::Graphics) -> Player {
        let sprite_map = Player::initialize_sprites(display);

        Player {
            sprites: sprite_map,
            x: x,
            y: y,
            velocity_x: 0.0,
            acceleration_x: 0.0,
            horizontal_facing: Left
        }
    }

    pub fn update(&mut self, elapsed_time: uint) {
        let sprite_state = self.get_sprite_state();
        self.sprites.get_mut(&sprite_state).update(elapsed_time);

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
        let sprite_state = self.get_sprite_state();
        self.sprites.get(&sprite_state).draw(display, self.x, self.y);
    }

    pub fn start_moving_left(&mut self) {
        self.acceleration_x = -WALKING_ACCELERATION;
        self.horizontal_facing = Left;
    }

    pub fn start_moving_right(&mut self) {
        self.acceleration_x = WALKING_ACCELERATION;
        self.horizontal_facing = Right;
    }

    pub fn stop_moving(&mut self) {
        self.acceleration_x = 0.0;
    }

    fn get_sprite_state(&self) -> SpriteState {
        let motion =
            if self.acceleration_x == 0.0 {
                Standing
            } else {
                Walking
            };
        let facing = self.horizontal_facing;

        SpriteState::new(motion, facing)
    }

    fn initialize_sprites(display: &mut graphics::Graphics) -> HashMap<SpriteState, ~sprite::HashableSprite:> {
        let mut sprite_map = HashMap::<SpriteState, ~sprite::HashableSprite:>::new();
        sprite_map.insert(SpriteState::new(Standing, Right), ~sprite::Sprite::new(
                ~"assets/charSprites.bmp", 0, TILE_SIZE, display) as ~sprite::HashableSprite:);
        sprite_map.insert(SpriteState::new(Walking, Right), ~animated_sprite::AnimatedSprite::new(
                ~"assets/charSprites.bmp", 0, TILE_SIZE, 15, 3, display) as ~sprite::HashableSprite:);
        sprite_map.insert(SpriteState::new(Standing, Left), ~sprite::Sprite::new(
                ~"assets/charSprites.bmp", 0, 0, display) as ~sprite::HashableSprite:);
        sprite_map.insert(SpriteState::new(Walking, Left), ~animated_sprite::AnimatedSprite::new(
                ~"assets/charSprites.bmp", 0, 0, 15, 3, display) as ~sprite::HashableSprite:);

        return sprite_map
    }
}
