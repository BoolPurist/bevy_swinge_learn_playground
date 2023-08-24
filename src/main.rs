use std::ops::Range;

use bevy::{input::common_conditions, prelude::*};
use bevy_inspector_egui::{
    quick::{ResourceInspectorPlugin, WorldInspectorPlugin},
    DefaultInspectorConfigPlugin,
};
use components::{Player, RandChangeWanderingNpc, Speed, WanderingNpc};
use plugins::GamePlugin;
use resources::{GameConfig, GAME_HEIGHT, GAME_WIDTH};

mod components;
pub mod pig_utils;
mod plugins;
mod rand_utils;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Logic Farming Rougelike".into(),
                        resolution: (GAME_WIDTH, GAME_HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            DefaultInspectorConfigPlugin,
            WorldInspectorPlugin::default().run_if(common_conditions::input_toggle_active(
                false,
                KeyCode::Numpad1,
            )),
            ResourceInspectorPlugin::<GameConfig>::default().run_if(
                common_conditions::input_toggle_active(false, KeyCode::Escape),
            ),
            GamePlugin,
        ))
        .register_type::<Range<f32>>()
        .run();
}
