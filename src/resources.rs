use std::ops::Range;

use bevy::{prelude::Resource, reflect::Reflect, time::Timer};

pub const GAME_HEIGHT: f32 = 720.0;
pub const GAME_WIDTH: f32 = 1080.0;

#[derive(Resource, Reflect)]
pub struct GameConfig {
    pub player_speed: f32,
    pub pig_min_life_time: f32,
    pub pig_max_life_time: f32,
    pub npc_rand_min_speed: f32,
    pub npc_rand_max_speed: f32,
    pub rand_state_change_time_min: f32,
    pub rand_state_change_time_max: f32,
    pub price: f32,
    pub income_amount: f32,
    pub income_time: f32,
}

impl GameConfig {
    pub fn rand_state_change_range(&self) -> Range<f32> {
        self.rand_state_change_time_min..self.rand_state_change_time_max
    }
    pub fn npc_rand_range_speed(&self) -> Range<f32> {
        self.npc_rand_min_speed..self.npc_rand_max_speed
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            player_speed: 500.0,
            pig_min_life_time: 8.,
            pig_max_life_time: 16.,
            npc_rand_min_speed: 50.,
            npc_rand_max_speed: 150.,
            rand_state_change_time_min: 1.0,
            rand_state_change_time_max: 2.0,
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
