use std::ops::Range;

use bevy::{prelude::Resource, reflect::Reflect, time::Timer};

pub const GAME_HEIGHT: f32 = 720.0;
pub const GAME_WIDTH: f32 = 1080.0;

#[derive(Resource, Reflect)]
pub struct GameConfig {
    pub player_speed: f32,
    pub pig_min_life_time: f32,
    pub pig_max_life_time: f32,
    pub npc_rand_speed_range: Range<f32>,
    pub random_state_time_change: Range<f32>,
    pub price: f32,
    pub income_amount: f32,
    pub income_time: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            player_speed: 500.0,
            pig_min_life_time: 8.,
            pig_max_life_time: 16.,
            npc_rand_speed_range: (50.0..150.0),
            random_state_time_change: (1.0..2.0),
            price: 10.0,
            income_amount: 15.0,
            income_time: 2.0,
        }
    }
}

#[derive(Resource)]
pub struct WorldDim {
    pub width: f32,
    pub height: f32,
}

#[derive(Resource)]
pub struct Money(pub f32);
#[derive(Resource)]
pub struct Income {
    pub when: Timer,
    pub amount: f32,
}
