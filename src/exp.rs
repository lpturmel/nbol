use crate::enemy::EnemyDefeatedEvent;
use crate::player::Player;
use bevy::prelude::*;

const BASE_EXP: f32 = 100.0;
const EXP_MULTIPLIER: f32 = 1.5;

pub struct ExperiencePlugin;

impl Plugin for ExperiencePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Experience>()
            .register_type::<Level>()
            .add_systems(Startup, setup_xp_ui)
            .add_systems(Update, award_experience)
            .add_systems(Update, update_xp_ui);
    }
}

#[derive(Component)]
pub struct ExperienceBar;

#[derive(Component)]
pub struct LevelText;

#[derive(Component)]
pub struct ExperienceText;

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct Level(u32);

impl Level {
    pub fn new(level: u32) -> Self {
        Self(level)
    }
    pub fn update(&mut self, level: u32) {
        self.0 = level;
    }
    pub fn get(&self) -> u32 {
        self.0
    }
}

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct Experience(f32);

impl Experience {
    pub fn new(current_xp: f32) -> Self {
        Self(current_xp)
    }
    pub fn current(&self) -> f32 {
        self.0
    }
    pub fn xp_to_next_level(&self, level: u32) -> f32 {
        BASE_EXP * (EXP_MULTIPLIER * level as f32)
    }
}

fn update_xp_ui(
    q_player: Query<(&Experience, &Level), With<Player>>,
    mut q_xp_text: Query<&mut Text, With<ExperienceText>>,
    mut q_level_text: Query<&mut Text, (With<LevelText>, Without<ExperienceText>)>,
    mut q_xp_bar: Query<&mut Style, (With<ExperienceBar>, Without<LevelText>)>,
) {
    let (player_xp, level) = q_player.single();
    for mut level_text in q_level_text.iter_mut() {
        level_text.sections[0].value = format!("{}", level.get());
    }
    for mut xp_text in q_xp_text.iter_mut() {
        xp_text.sections[0].value = format!(
            "{} / {}",
            player_xp.current(),
            player_xp.xp_to_next_level(level.get())
        );
    }
    for mut xp_bar in q_xp_bar.iter_mut() {
        let xp_needed = player_xp.xp_to_next_level(level.get());
        let xp_percentage = player_xp.current() / xp_needed;
        xp_bar.width = Val::Percent(xp_percentage * 100.0);
    }
}

fn award_experience(
    mut events: EventReader<EnemyDefeatedEvent>,
    mut q_player: Query<(&mut Experience, &mut Level), With<Player>>,
) {
    let (mut player_xp, mut level) = q_player.single_mut();
    for _ in events.read() {
        let xp_awarded = 25.0; // TODO: Calculate based on enemy level
                               //
        let xp_needed = player_xp.xp_to_next_level(**level);

        **player_xp += xp_awarded;

        if player_xp.current() >= xp_needed {
            **level += 1;
            **player_xp = 0.0;
        }
    }
}
fn setup_xp_ui(mut commands: Commands) {
    let container = NodeBundle {
        style: Style {
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            display: Display::Flex,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            flex_direction: FlexDirection::Row,
            row_gap: Val::Px(5.0),
            width: Val::Percent(100.0),
            height: Val::Px(20.0),
            ..Default::default()
        },
        ..Default::default()
    };
    let container_id = commands.spawn(container).id();
    let level = Level::new(1);
    let experience = Experience::new(0.0);
    let level_text = TextBundle::from_section(
        format!("{}", level.get()),
        TextStyle {
            color: Color::WHITE,
            font_size: 15.0,
            ..default()
        },
    );
    let level_text_entity = commands.spawn((level_text, LevelText)).id();
    let xp_bar_background = NodeBundle {
        style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            position_type: PositionType::Relative,
            height: Val::Percent(100.0),
            ..Default::default()
        },
        background_color: Color::BLACK.into(),
        ..Default::default()
    };
    let xp_bar_bg_entity = commands.spawn(xp_bar_background).id();
    let experience_text = TextBundle::from_section(
        format!(
            "{} / {}",
            experience.current(),
            experience.xp_to_next_level(level.get())
        ),
        TextStyle::default(),
    );
    let experience_text_entity = commands.spawn((experience_text, ExperienceText)).id();
    let experience_bar_entity = NodeBundle {
        style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            // TODO - change to 0.0
            width: Val::Percent(0.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            ..Default::default()
        },
        background_color: Color::BLUE.into(),
        ..Default::default()
    };
    let experience_bar_entity = commands.spawn((experience_bar_entity, ExperienceBar)).id();

    commands.entity(container_id).add_child(level_text_entity);
    commands.entity(container_id).add_child(xp_bar_bg_entity);

    commands
        .entity(xp_bar_bg_entity)
        .add_child(experience_bar_entity);

    commands
        .entity(xp_bar_bg_entity)
        .add_child(experience_text_entity);
}
