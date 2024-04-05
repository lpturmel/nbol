use crate::player::Player;
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_player);
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
