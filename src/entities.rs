use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct SpriteSheet {
    pub handle: Handle<Image>,
    pub up: Vec<usize>,
    pub down: Vec<usize>,
    pub left: Vec<usize>,
    pub right: Vec<usize>,
}

#[derive(Component, Debug, Copy, Clone)]
pub enum Facing {
    Up,
    Down,
    Left,
    Right,
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

#[derive(Component, Debug, Deref, DerefMut)]
pub struct DespawnTimer(pub Timer);

#[derive(Component)]
pub struct Health {
    current: f32,
    max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
    pub fn current(&self) -> f32 {
        self.current
    }
    pub fn max(&self) -> f32 {
        self.max
    }
    pub fn update(&mut self, damage: f32) {
        self.current -= damage;
    }
}

#[derive(Event)]
pub struct HealthUpdateEvent {
    pub entity: Entity,
    pub total_health: f32,
    pub new_health: f32,
}
