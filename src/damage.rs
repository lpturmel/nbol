use crate::enemy::EnemyDefeatedEvent;
use crate::entities::{Health, HealthUpdateEvent};
use bevy::prelude::*;

#[derive(Event)]
pub struct DamageEvent {
    pub damage: f32,
    pub crit_hit: CriticalHit,
    pub entity: Entity,
}
#[derive(Event)]
pub struct DisplayDamageNumbersEvent {
    pub damage: f32,
    pub position: Transform,
    pub is_crit: bool,
}

#[derive(Component, Deref, DerefMut)]
/// Defines the damage of an entity
pub struct Damage(f32);

impl Damage {
    pub fn new(damage: f32) -> Self {
        Self(damage)
    }
}

#[derive(Component, Clone, Copy)]
/// Defines a critical hit chance and multiplier of an entity
pub struct CriticalHit {
    pub chance: f32,
    pub multiplier: f32,
}

impl CriticalHit {
    pub fn new(chance: f32, multiplier: f32) -> Self {
        Self { chance, multiplier }
    }
}
pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_event::<DisplayDamageNumbersEvent>()
            .add_event::<HealthUpdateEvent>()
            .add_systems(Update, apply_damage_system);
    }
}

fn apply_damage_system(
    mut commands: Commands,
    mut damage_events: EventReader<DamageEvent>,
    mut display_damage_events: EventWriter<DisplayDamageNumbersEvent>,
    mut health_update_events: EventWriter<HealthUpdateEvent>,
    mut enemy_events: EventWriter<EnemyDefeatedEvent>,
    mut q_health: Query<(Entity, &Transform, &mut Health)>,
) {
    for e in damage_events.read() {
        if let Ok((entity, transform, mut health)) = q_health.get_mut(e.entity) {
            let mut final_damage = e.damage;
            if e.crit_hit.chance > 0.0 {
                let random = rand::random::<f32>();
                if random <= e.crit_hit.chance {
                    final_damage *= e.crit_hit.multiplier;
                }
            }
            health.update(final_damage);
            display_damage_events.send(DisplayDamageNumbersEvent {
                damage: final_damage,
                position: *transform,
                is_crit: final_damage != e.damage,
            });
            health_update_events.send(HealthUpdateEvent {
                entity,
                total_health: health.max(),
                new_health: health.current(),
            });
            if health.current() <= 0.0 {
                enemy_events.send(EnemyDefeatedEvent);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
