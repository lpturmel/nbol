use crate::abilities::AbilityPlugin;
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;
use crate::splash::SplashPlugin;
use crate::world::WorldPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod abilities;
pub mod enemy;
pub mod entities;
pub mod player;
pub mod world;

pub const TILE_SIZE: f32 = 64.0;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    InGame,
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(AbilityPlugin)
            .add_plugins(WorldPlugin)
            .add_plugins(WorldInspectorPlugin::new());
    }
}
fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Nbol".into(),
                name: Some("nbol".into()),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, spawn_camera)
        .add_plugins((SplashPlugin, GamePlugin))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
mod splash {
    use crate::{despawn_screen, GameState};
    use bevy::prelude::*;

    pub struct SplashPlugin;

    impl Plugin for SplashPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::Splash), setup_splash)
                .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
                .add_systems(OnExit(GameState::Splash), despawn_screen::<SplashScreen>);
        }
    }

    #[derive(Component)]
    struct SplashScreen;

    #[derive(Resource, Deref, DerefMut)]
    struct SplashTimer(Timer);

    fn setup_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
        let splash = asset_server.load("splash.png");

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        column_gap: Val::Px(10.0),
                        display: Display::Flex,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                },
                SplashScreen,
            ))
            .with_children(|p| {
                p.spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        ..default()
                    },
                    image: UiImage::new(splash),
                    ..default()
                });
                p.spawn(TextBundle::from_section(
                    "Nbol",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
    }

    fn countdown(
        mut game_state: ResMut<NextState<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(GameState::InGame)
        }
    }
}
