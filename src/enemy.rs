use crate::entities::{get_facing_direction, Facing, FrameAnimation, Health, HealthUpdateEvent};
use crate::player::Player;
use crate::TILE_SIZE;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::{thread_rng, Rng};

const COLUMNS: usize = 13;
const ROWS: usize = 21;
const ENEMY_COUNT: usize = 10;
const ENEMY_FRAMES: usize = 9;

#[derive(Component)]
pub struct EnemyNameUI;
#[derive(Component)]
pub struct EnemyHealthBackgroundUI;
#[derive(Component)]
pub struct EnemyHealthForegroundUI;

pub fn update_hp_ui(sprite: &mut Sprite, total_health: f32, new_health: f32) {
    let percentage = new_health / total_health;
    let foreground_scale_x = TILE_SIZE * percentage;
    *sprite = Sprite {
        color: Color::RED,
        custom_size: Some(Vec2::new(foreground_scale_x, 5.0)),
        ..default()
    };
}

#[derive(Resource, Clone)]
pub struct SkeletonSheet {
    pub handle: Handle<Image>,
    pub up: Vec<usize>,
    pub down: Vec<usize>,
    pub left: Vec<usize>,
    pub right: Vec<usize>,
}
#[derive(Component, Debug)]
pub struct Enemy {
    pub speed: f32,
    pub level: u32,
    pub moving: bool,
    pub aggro_range: f32,
    pub aggro: bool,
    pub spawn_coords: Vec3,
    pub display_name: String,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            spawn_coords: Vec3::new(0.0, 0.0, 0.0),
            aggro: false,
            aggro_range: 150.0,
            speed: 1.0,
            moving: false,
            display_name: "".to_string(),
            level: 3,
        }
    }
}
#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub sprite: SpriteSheetBundle,
    pub animation: FrameAnimation,
    pub facing: Facing,
    pub health: Health,
    // pub ui: EnemyUI,
}

#[derive(Component)]
pub struct EnemyUI {
    pub name: EnemyNameUI,
    // pub health: EnemyUIHealth,
}

#[derive(Component)]
pub struct EnemyUIHealth {
    pub background: EnemyHealthBackgroundUI,
    pub foreground: EnemyHealthForegroundUI,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies)
            .add_systems(Update, move_enemies)
            .add_systems(Update, animate_enemies)
            .add_systems(Update, update_enemy_graphics)
            .add_systems(Update, update_health_ui);
    }
}

fn update_health_ui(
    mut events: EventReader<HealthUpdateEvent>,
    mut q_sprites: Query<(&mut Sprite, &Parent), With<EnemyHealthForegroundUI>>,
    mut q_background: Query<(&Parent, &Children), With<EnemyHealthBackgroundUI>>,
) {
    for event in events.read() {
        for (parent, children) in q_background.iter_mut() {
            if parent.get() == event.entity {
                for child in children.iter() {
                    if let Ok(mut sprite) = q_sprites.get_mut(*child) {
                        update_hp_ui(&mut sprite.0, event.total_health, event.new_health);
                    }
                }
            }
        }
    }
}
fn animate_enemies(
    mut sprites_query: Query<(&mut TextureAtlas, &Enemy, &mut FrameAnimation), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut texture_atlas, enemy, mut animation) in &mut sprites_query.iter_mut() {
        if enemy.moving {
            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
                texture_atlas.index = animation.frames[animation.current_frame];
            }
        } else {
            texture_atlas.index = animation.frames[0];
            animation.current_frame = 0;
        }
    }
}

fn update_enemy_graphics(
    mut sprites_query: Query<(&Facing, &mut FrameAnimation), With<Enemy>>,
    char: Res<SkeletonSheet>,
) {
    for (facing, mut animation) in &mut sprites_query.iter_mut() {
        animation.frames = match facing {
            Facing::Up => char.up.to_vec(),
            Facing::Down => char.down.to_vec(),
            Facing::Left => char.left.to_vec(),
            Facing::Right => char.right.to_vec(),
        };
    }
}
fn move_enemies(
    mut query: Query<(&mut Transform, &mut Facing, &mut Enemy, &mut FrameAnimation), With<Enemy>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player.single();
    for (mut transform, mut facing, mut enemy, mut animation) in &mut query.iter_mut() {
        let direction = player_transform.translation - transform.translation;
        let distance = direction.length();

        let direction_to_spawn = enemy.spawn_coords - transform.translation;
        let distance_to_spawn = direction_to_spawn.length();

        let aggro_range = if enemy.aggro {
            enemy.aggro_range * 3.0
        } else {
            enemy.aggro_range
        };
        if distance <= aggro_range {
            let direction = direction / distance;
            let movement = direction * enemy.speed;
            *facing = get_facing_direction(direction);
            transform.translation += movement;
            enemy.moving = true;
            enemy.aggro = false;
            animation.timer.unpause();
        } else if distance_to_spawn > 2.0 {
            let direction = direction_to_spawn / distance_to_spawn;
            let movement = direction * enemy.speed * 4.0; // TODO evaluate: Move back multiplier
            *facing = get_facing_direction(direction);
            transform.translation += movement;
            enemy.moving = true;
            enemy.aggro = false;
            animation.timer.unpause();
        } else {
            enemy.moving = false;
            *facing = Facing::Down;
            animation.timer.pause();
        }
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

    let enemy_sheet = SkeletonSheet {
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
        let coordinates = Vec3::new(x, y, 0.0);
        let enemy_transform = Transform::from_translation(coordinates);
        let sprite_bundle = SpriteSheetBundle {
            texture: texture_handle.clone(),
            atlas: TextureAtlas {
                layout: atlas_handle.clone(),
                index: initial_frame,
            },
            transform: enemy_transform,
            ..Default::default()
        };

        let enemy = Enemy {
            spawn_coords: coordinates,
            display_name: "Skeleton".to_string(),
            ..Default::default()
        };

        let offset_y = TILE_SIZE;
        let ui_transform = Transform::from_translation(Vec3::new(0.0, offset_y / 1.5, 0.0));

        let name_ui = Text2dBundle {
            text: Text::from_section(
                enemy.display_name.clone(),
                TextStyle {
                    font_size: 15.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            transform: ui_transform,
            ..default()
        };
        let health_ui = Transform::from_translation(Vec3::new(0.0, 32.0, 0.0));
        let background_ui = SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(TILE_SIZE, 5.0)),
                ..default()
            },
            transform: health_ui,
            ..default()
        };

        let enemy_health = Health::new(130.0);
        let enemy_health_percentage = enemy_health.current() / enemy_health.max();
        let foreground_scale_x = TILE_SIZE * enemy_health_percentage;
        let foreground_ui = SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(foreground_scale_x, 5.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                -TILE_SIZE / 2.0 + foreground_scale_x / 2.0,
                0.0,
                0.0,
            )),
            ..default()
        };
        let enemy = EnemyBundle {
            enemy,
            facing,
            health: Health::new(130.0),
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
        commands
            .spawn(enemy)
            .insert(Name::new("enemy"))
            .with_children(|p| {
                p.spawn((name_ui, EnemyNameUI));
                p.spawn((background_ui, EnemyHealthBackgroundUI))
                    .with_children(|p| {
                        p.spawn((foreground_ui, EnemyHealthForegroundUI));
                    });
            });
    }
}
