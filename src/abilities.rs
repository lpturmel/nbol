use crate::entities::{Facing, FrameAnimation, Graphics};
use crate::TILE_SIZE;
use bevy::prelude::*;

const COLUMNS: usize = 8;
const ROWS: usize = 8;

const FIREBALL_FRAMES: usize = 7;

#[derive(Resource)]
pub struct Abilities {
    pub fireball: FireballRes,
}
#[derive(Resource)]
pub struct FireballRes {
    pub handle: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub up: Vec<usize>,
    pub down: Vec<usize>,
    pub left: Vec<usize>,
    pub right: Vec<usize>,
}

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_abilities)
            .add_systems(Update, animate_fireball)
            .add_systems(Update, projectile_mouvement)
            .add_systems(Update, projectile_despawn);
    }
}

#[derive(Component)]
pub struct Fireball;

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    pub damage: f32,
    pub moving: bool,
    pub graphics: Graphics,
}

impl Projectile {
    pub fn new(direction: Facing) -> Self {
        Self {
            speed: 7.5,
            damage: 10.0,
            moving: false,
            graphics: Graphics { facing: direction },
        }
    }
}

#[derive(Component)]
pub struct AbilityDespawnTimer(pub Timer);

fn load_abilities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("fireball.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(64.0, 64.0), COLUMNS, ROWS, None, None);
    let texture_atlas_layout = textures.add(layout);
    let row_start = 0;
    let left: Vec<usize> = (0..FIREBALL_FRAMES)
        .map(|i| COLUMNS * row_start + i)
        .collect::<Vec<_>>();
    let up: Vec<usize> = (0..FIREBALL_FRAMES)
        .map(|i| COLUMNS * (row_start + 2) + i)
        .collect::<Vec<_>>();
    let right: Vec<usize> = (0..FIREBALL_FRAMES)
        .map(|i| COLUMNS * (row_start + 4) + i)
        .collect::<Vec<_>>();
    let down: Vec<usize> = (0..FIREBALL_FRAMES)
        .map(|i| COLUMNS * (row_start + 6) + i)
        .collect::<Vec<_>>();

    let fireball_sheet = FireballRes {
        handle: texture_handle,
        layout: texture_atlas_layout,
        up,
        down,
        left,
        right,
    };
    commands.insert_resource(Abilities {
        fireball: fireball_sheet,
    });
}

fn projectile_despawn(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AbilityDespawnTimer), With<Fireball>>,
    time: Res<Time>,
) {
    for (entity, mut timer) in query.iter_mut() {
        if timer.0.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        }
    }
}
fn projectile_mouvement(mut query: Query<(&Projectile, &mut Transform)>, time: Res<Time>) {
    let projectiles = query.iter_mut();
    for (projectile, mut transform) in projectiles {
        let delta = match projectile.graphics.facing {
            Facing::Up => Vec3::new(
                0.0,
                projectile.speed * TILE_SIZE * time.delta_seconds(),
                0.0,
            ),
            Facing::Down => Vec3::new(
                0.0,
                -projectile.speed * TILE_SIZE * time.delta_seconds(),
                0.0,
            ),
            Facing::Left => Vec3::new(
                -projectile.speed * TILE_SIZE * time.delta_seconds(),
                0.0,
                0.0,
            ),
            Facing::Right => Vec3::new(
                projectile.speed * TILE_SIZE * time.delta_seconds(),
                0.0,
                0.0,
            ),
        };
        let target = transform.translation + delta;
        transform.translation = target;
    }
}
fn animate_fireball(
    mut sprites_query: Query<(&mut TextureAtlas, &mut FrameAnimation), With<Fireball>>,
    time: Res<Time>,
) {
    for (mut texture_atlas, mut animation) in sprites_query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
            texture_atlas.index = animation.frames[animation.current_frame];
        }
    }
}
