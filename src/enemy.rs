use crate::entities::{Facing, FrameAnimation, Graphics, SpriteSheet};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::{thread_rng, Rng};

const COLUMNS: usize = 13;
const ROWS: usize = 21;

const ENEMY_COUNT: usize = 10;

const ENEMY_FRAMES: usize = 9;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub health: f32,
    pub moving: bool,
    pub graphics: Graphics,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub sprite: SpriteSheetBundle,
    pub animation: FrameAnimation,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            speed: 0.5,
            health: 100.0,
            moving: false,
            graphics: Graphics {
                facing: Facing::Down,
            },
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies);
    }
}

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlasLayout>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();
    let texture_handle = asset_server.load("enemy.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(64.0, 64.0), COLUMNS, ROWS, None, None);
    let atlas_handle = textures.add(layout);
    let row_start = 8;
    let enemy_up: Vec<usize> = (0..ENEMY_FRAMES)
        .map(|i| COLUMNS * row_start + i)
        .collect::<Vec<_>>();
    let enemy_left: Vec<usize> = (0..ENEMY_FRAMES)
        .map(|i| COLUMNS * (row_start + 1) + i)
        .collect::<Vec<_>>();
    let enemy_down: Vec<usize> = (0..ENEMY_FRAMES)
        .map(|i| COLUMNS * (row_start + 2) + i)
        .collect::<Vec<_>>();
    let enemy_right: Vec<usize> = (0..ENEMY_FRAMES)
        .map(|i| COLUMNS * (row_start + 3) + i)
        .collect::<Vec<_>>();

    let enemy_sheet = SpriteSheet {
        handle: texture_handle.clone(),
        up: enemy_up.clone(),
        down: enemy_down.clone(),
        left: enemy_left.clone(),
        right: enemy_right.clone(),
    };
    commands.insert_resource(enemy_sheet);

    let mut rng = thread_rng();
    for _ in 0..ENEMY_COUNT {
        let x = rng.gen_range(0.0..window.width());
        let y = rng.gen_range(0.0..window.height());
        let facing = match rng.gen_range(0..4) {
            0 => Facing::Up,
            1 => Facing::Down,
            2 => Facing::Left,
            3 => Facing::Right,
            _ => unreachable!(),
        };
        let initial_frame = match facing {
            Facing::Up => enemy_up[0],
            Facing::Down => enemy_down[0],
            Facing::Left => enemy_left[0],
            Facing::Right => enemy_right[0],
        };
        let sprite_bundle = SpriteSheetBundle {
            texture: texture_handle.clone(),
            atlas: TextureAtlas {
                layout: atlas_handle.clone(),
                index: initial_frame,
            },
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            ..Default::default()
        };
        let enemy = EnemyBundle {
            enemy: Enemy {
                graphics: Graphics {
                    facing: facing.clone(),
                },
                ..Default::default()
            },
            sprite: sprite_bundle,
            animation: FrameAnimation {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                frames: match facing {
                    Facing::Up => enemy_up.to_vec(),
                    Facing::Down => enemy_down.to_vec(),
                    Facing::Left => enemy_left.to_vec(),
                    Facing::Right => enemy_right.to_vec(),
                },
                current_frame: 0,
            },
        };
        commands.spawn(enemy);
    }
}
