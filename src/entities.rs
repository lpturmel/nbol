use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct SpriteSheet {
    pub handle: Handle<Image>,
    pub up: Vec<usize>,
    pub down: Vec<usize>,
    pub left: Vec<usize>,
    pub right: Vec<usize>,
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub enum Facing {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Graphics {
    pub facing: Facing,
}

#[derive(Component)]
pub struct FrameAnimation {
    pub timer: Timer,
    pub frames: Vec<usize>,
    pub current_frame: usize,
}

/// Returns the facing direction based on the given vector
pub fn get_facing_direction(direction: Vec3) -> Facing {
    if direction.x.abs() > direction.y.abs() {
        if direction.x > 0.0 {
            Facing::Right
        } else {
            Facing::Left
        }
    } else if direction.y > 0.0 {
        Facing::Up
    } else {
        Facing::Down
    }
}
