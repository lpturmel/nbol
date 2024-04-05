use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct SpriteSheet {
    pub handle: Handle<Image>,
    pub up: Vec<usize>,
    pub down: Vec<usize>,
    pub left: Vec<usize>,
    pub right: Vec<usize>,
}

#[derive(Component, Debug, Clone)]
pub enum Facing {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Clone)]
pub struct Graphics {
    pub facing: Facing,
}

#[derive(Component)]
pub struct FrameAnimation {
    pub timer: Timer,
    pub frames: Vec<usize>,
    pub current_frame: usize,
}
