use crate::damage::{CriticalHit, Damage, DamageEvent, DisplayDamageNumbersEvent};
use crate::enemy::Enemy;
use crate::entities::{DespawnTimer, Facing, FrameAnimation};
use crate::TILE_SIZE;
use bevy::prelude::*;
use rand::Rng;

const COLUMNS: usize = 8;
const ROWS: usize = 8;

const FIREBALL_FRAMES: usize = 7;

#[derive(Resource)]
pub struct AbilitySheet {
    pub fireball: FireballSheet,
}
#[derive(Resource)]
pub struct FireballSheet {
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
            .add_systems(Update, projectile_collision)
            .add_systems(Update, display_damage_numbers);
    }
}

#[derive(Component)]
pub struct Fireball;

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    pub damage: f32,
    pub moving: bool,
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            speed: 7.5,
            damage: 35.0,
            moving: false,
        }
    }
}

fn projectile_collision(
    mut commands: Commands,
    mut q_projectiles: Query<(Entity, &Transform, &Damage, &CriticalHit), With<Projectile>>,
    mut q_enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut ev_damage: EventWriter<DamageEvent>,
) {
    for (projectile_entity, projectile_transform, damage, crit) in q_projectiles.iter_mut() {
        for (enemy_entity, enemy_transform) in q_enemies.iter_mut() {
            let distance = enemy_transform
                .translation
                .distance(projectile_transform.translation);
            if distance < (TILE_SIZE * 0.75) {
                commands.entity(projectile_entity).despawn_recursive();
                ev_damage.send(DamageEvent {
                    damage: **damage,
                    crit_hit: *crit,
                    entity: enemy_entity,
                });
                break;
            }
        }
    }
}
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

    let fireball_sheet = FireballSheet {
        handle: texture_handle,
        layout: texture_atlas_layout,
        up,
        down,
        left,
        right,
    };
    commands.insert_resource(AbilitySheet {
        fireball: fireball_sheet,
    });
}

fn projectile_mouvement(mut query: Query<(&Projectile, &Facing, &mut Transform)>, time: Res<Time>) {
    let projectiles = query.iter_mut();
    for (projectile, facing, mut transform) in projectiles {
        let delta = match facing {
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

fn display_damage_numbers(
    mut commands: Commands,
    mut events: EventReader<DisplayDamageNumbersEvent>,
) {
    for event in events.read() {
        let mut rng = rand::thread_rng();
        let x = event.position.translation.x + rng.gen_range(-10.0..10.0);
        let y = event.position.translation.y + rng.gen_range(-10.0..10.0);
        let color = if event.is_crit {
            Color::ORANGE
        } else {
            Color::WHITE
        };
        let font_size = if event.is_crit { 24.0 } else { 20.0 };
        commands
            .spawn(Text2dBundle {
                text: Text::from_section(
                    format!("{:.0}", event.damage),
                    TextStyle {
                        font: Handle::default(),
                        font_size,
                        color,
                    },
                ),
                transform: Transform::from_xyz(x, y, 2.0),
                ..Default::default()
            })
            .insert(DespawnTimer(Timer::from_seconds(0.5, TimerMode::Once)));
    }
}
