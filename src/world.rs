use crate::entities::DespawnTimer;
use crate::player::Player;
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_player)
            .add_systems(Update, despawn_timed_entities);
    }
}
fn despawn_timed_entities(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DespawnTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in query.iter_mut() {
        if timer.tick(time.delta()).finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn follow_player(
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let player_transform = player.single();
    let mut camera_transform = camera.single_mut();
    camera_transform.translation = player_transform.translation;
}
