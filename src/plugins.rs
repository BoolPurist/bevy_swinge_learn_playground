use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use crate::{
    components::{AabbColl, MainGameCamera, MoneyScore, PigContainer},
    resources::{GameConfig, Income, Money, WorldDim, GAME_HEIGHT, GAME_WIDTH},
    systems, Player, Speed,
};
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let game_config = GameConfig::default();
        let dimesion = WorldDim {
            width: GAME_WIDTH,
            height: GAME_HEIGHT,
        };
        app.insert_resource(Income {
            when: Timer::from_seconds(game_config.income_time, TimerMode::Repeating),
            amount: game_config.income_amount,
        })
        .insert_resource(game_config)
        .insert_resource(dimesion)
        .register_type::<GameConfig>()
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                systems::character_movement,
                systems::kill_pig,
                systems::spawn_pig,
                systems::pig_movement,
                systems::flip_wandering_npc,
                systems::handle_income,
                systems::keep_inside_window,
                systems::scale_player,
                systems::change_money_score,
                systems::change_state_on_rand,
            ),
        );
    }
}

fn setup(mut commands: Commands, config: Res<GameConfig>, assert_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::DARK_GREEN),
            },
            ..default()
        },
        MainGameCamera,
    ));
    commands.spawn((
        SpatialBundle::default(),
        Name::new("All pigs"),
        PigContainer,
    ));

    commands.spawn((
        TextBundle::from_section(
            "Money: ?",
            TextStyle {
                font_size: 50.0,
                color: Color::BLACK,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        MoneyScore,
    ));

    let texture = assert_server.load("character.png");
    let player_bounds = Vec2::new(100.0, 100.0);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(player_bounds),
                ..default()
            },
            texture,
            ..default()
        },
        Player,
        Speed(config.player_speed),
        AabbColl,
        Name::new("Player"),
    ));
}

fn _spawn_retangle(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            color: Color::RED,
            ..default()
        },
        ..default()
    });
}
