use crate::abilities::{AbilitySheet, Fireball, Projectile};
use crate::damage::{CriticalHit, Damage};
use crate::entities::{DespawnTimer, Facing, FrameAnimation, Health, SpriteSheet};
use crate::TILE_SIZE;
use bevy::prelude::*;

/// Player sprite animation frames
const PLAYER_FRAMES: usize = 9;
/// Player base movement speed
const PLAYER_SPEED: f32 = 2.0;
/// Player sprite size columns
const COLUMNS: usize = 13;
/// Player sprite size rows
const ROWS: usize = 21;

const ENERGY_RECOVERY: f32 = 15.0; // per second
const ENERGY_COST: f32 = 10.0; // per second

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub sprite: SpriteSheetBundle,
    pub animation: FrameAnimation,
    pub facing: Facing,
    pub critical_hit: CriticalHit,
    pub damage: Damage,
}
#[derive(Component)]
pub struct Player {
    pub level: u32,
    pub speed: f32,
    pub moving: bool,
    pub energy: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            level: 1,
            speed: PLAYER_SPEED,
            moving: false,
            energy: 100.0,
        }
    }
}

#[derive(Component)]
pub struct LevelDisplay;

fn setup_level_ui(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Level: 1",
            TextStyle {
                color: Color::GOLD,
                font_size: 20.0,
                ..default()
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        LevelDisplay,
    ));
}
#[derive(Component)]
pub struct EnergyDisplay;

fn setup_energy_ui(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Energy: 100",
            TextStyle {
                color: Color::WHITE,
                font_size: 20.0,
                ..default()
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            top: Val::Px(40.0),
            left: Val::Px(10.0),
            ..default()
        }),
        EnergyDisplay,
    ));
}
fn update_energy_ui(
    mut ui_query: Query<&mut Text, With<EnergyDisplay>>,
    player_query: Query<&Player>,
) {
    let player = player_query.single();
    let mut energy_text = ui_query.single_mut();
    energy_text.sections[0].value = format!("Energy: {:.0}", player.energy);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Startup, setup_energy_ui)
            .add_systems(Startup, setup_level_ui)
            .add_systems(Update, player_mouvement)
            .add_systems(Update, animate_player)
            .add_systems(Update, update_player_graphics)
            .add_systems(Update, energy_system)
            .add_systems(Update, update_energy_ui)
            .add_systems(Update, throw_fireball);
    }
}

fn throw_fireball(
    mut commands: Commands,
    keyboard: Res<ButtonInput<MouseButton>>,
    player_query: Query<(&Facing, &Transform), With<Player>>,
    abilities: Res<AbilitySheet>,
) {
    if keyboard.just_pressed(MouseButton::Right) {
        let (facing, transform) = player_query.single();
        let direction = facing;
        let projectile = Projectile::default();
        let player_coords = transform.translation;
        let initial_frame = match direction {
            Facing::Up => abilities.fireball.up[0],
            Facing::Down => abilities.fireball.down[0],
            Facing::Left => abilities.fireball.left[0],
            Facing::Right => abilities.fireball.right[0],
        };
        let sprite_bundle = SpriteSheetBundle {
            texture: abilities.fireball.handle.clone(),
            atlas: TextureAtlas {
                layout: abilities.fireball.layout.clone(),
                index: initial_frame,
            },
            transform: Transform::from_translation(player_coords),
            ..default()
        };
        commands
            .spawn((
                projectile,
                sprite_bundle,
                FrameAnimation {
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    frames: match direction {
                        Facing::Up => abilities.fireball.up.to_vec(),
                        Facing::Down => abilities.fireball.down.to_vec(),
                        Facing::Left => abilities.fireball.left.to_vec(),
                        Facing::Right => abilities.fireball.right.to_vec(),
                    },
                    current_frame: 0,
                },
                Fireball,
                DespawnTimer(Timer::from_seconds(5.0, TimerMode::Once)),
                Damage::new(20.0),
                // 10% chance to deal double damage
                CriticalHit::new(0.1, 2.0),
                *facing,
            ))
            .insert(Name::new("fireball"));
    }
}
fn energy_system(
    mut player_query: Query<(&mut Player, &Transform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, _) = player_query.single_mut();
    if keyboard.pressed(KeyCode::ShiftLeft) && player.moving {
        player.energy -= ENERGY_COST * time.delta_seconds();
        if player.energy < 0.0 {
            player.energy = 0.0;
        }
    } else {
        player.energy += ENERGY_RECOVERY * time.delta_seconds();
        if player.energy > 100.0 {
            player.energy = 100.0;
        }
    }
}
fn update_player_graphics(
    mut sprites_query: Query<(&Facing, &mut FrameAnimation), With<Player>>,
    char: Res<SpriteSheet>,
) {
    let (facing, mut animation) = sprites_query.single_mut();
    animation.frames = match facing {
        Facing::Up => char.up.to_vec(),
        Facing::Down => char.down.to_vec(),
        Facing::Left => char.left.to_vec(),
        Facing::Right => char.right.to_vec(),
    };
}
fn animate_player(
    mut sprites_query: Query<(&mut TextureAtlas, &mut FrameAnimation), With<Player>>,
    player_query: Query<&Player>,
    time: Res<Time>,
) {
    if player_query.single().moving {
        // single player
        let (mut texture_atlas, mut animation) = sprites_query.single_mut();
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
            texture_atlas.index = animation.frames[animation.current_frame];
        }
    } else {
        let (mut texture_atlas, mut animation) = sprites_query.single_mut();
        texture_atlas.index = animation.frames[0];
        animation.current_frame = 0;
    }
}

fn player_mouvement(
    mut player_query: Query<(&mut Player, &mut Facing, &mut Transform)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut facing, mut transform) = player_query.single_mut();
    player.moving = false;

    let speed_modif = if keyboard_input.pressed(KeyCode::ShiftLeft) && player.energy > 0.0 {
        1.5
    } else {
        1.0
    };

    let mut y_delta = 0.0;
    if keyboard_input.pressed(KeyCode::KeyW) {
        y_delta += player.speed * TILE_SIZE * time.delta_seconds() * speed_modif;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        y_delta -= player.speed * TILE_SIZE * time.delta_seconds() * speed_modif;
    }
    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);

    if y_delta != 0.0 {
        player.moving = true;
        if y_delta > 0.0 {
            *facing = Facing::Up;
        } else if y_delta < 0.0 {
            *facing = Facing::Down;
        }
        transform.translation = target;
    }
    let mut x_delta = 0.0;
    if keyboard_input.pressed(KeyCode::KeyA) {
        x_delta -= player.speed * TILE_SIZE * time.delta_seconds() * speed_modif;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        x_delta += player.speed * TILE_SIZE * time.delta_seconds() * speed_modif;
    }
    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);

    if x_delta != 0.0 {
        player.moving = true;
        if x_delta > 0.0 {
            *facing = Facing::Right;
        } else if x_delta < 0.0 {
            *facing = Facing::Left;
        }
        transform.translation = target;
    }
}
/// Spawns the player sprite
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("character.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(64.0, 64.0), COLUMNS, ROWS, None, None);
    let texture_atlas_layout = textures.add(layout);
    let row_start = 8;
    let player_up: Vec<usize> = (0..PLAYER_FRAMES)
        .map(|i| COLUMNS * row_start + i)
        .collect::<Vec<_>>();
    let player_left: Vec<usize> = (0..PLAYER_FRAMES)
        .map(|i| COLUMNS * (row_start + 1) + i)
        .collect::<Vec<_>>();
    let player_down: Vec<usize> = (0..PLAYER_FRAMES)
        .map(|i| COLUMNS * (row_start + 2) + i)
        .collect::<Vec<_>>();
    let player_right: Vec<usize> = (0..PLAYER_FRAMES)
        .map(|i| COLUMNS * (row_start + 3) + i)
        .collect::<Vec<_>>();

    let character_sheet = SpriteSheet {
        handle: texture_handle.clone(),
        up: player_up,
        down: player_down.clone(),
        left: player_left,
        right: player_right,
    };
    commands.insert_resource(character_sheet);

    let sprite_bundle = SpriteSheetBundle {
        texture: texture_handle,
        atlas: TextureAtlas {
            layout: texture_atlas_layout,
            index: 1,
        },
        transform: Transform::from_scale(Vec3::splat(1.0)),
        ..default()
    };
    let player = PlayerBundle {
        player: Player::default(),
        health: Health::new(500.0),
        sprite: sprite_bundle,
        animation: FrameAnimation {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frames: player_down.to_vec(),
            current_frame: 0,
        },
        facing: Facing::Down,
        critical_hit: CriticalHit::new(0.1, 2.0),
        damage: Damage::new(10.0),
    };
    commands.spawn(player).insert(Name::new("player"));
}
