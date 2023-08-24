use bevy::prelude::*;

use crate::{
    components::Pig, rand_utils, resources::GameConfig, RandChangeWanderingNpc, Speed, WanderingNpc,
};
use rand::Rng;

pub fn create_pig(
    player_trans: &Transform,
    commands: &mut Commands,
    assert_server: &AssetServer,
    config: &GameConfig,
) {
    let texture = assert_server.load("pig.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: player_trans.translation,
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            texture,
            ..default()
        },
        new_pig_random(config),
    ));
}

fn new_pig_random(config: &GameConfig) -> (Pig, Speed, WanderingNpc, RandChangeWanderingNpc) {
    let mut rand_seed = rand::thread_rng();
    let random_life_time = rand_seed.gen_range(config.pig_min_life_time..config.pig_max_life_time);
    let rand_change_state = rand_seed.gen_range(config.random_state_time_change.clone());
    let rand_speed = rand_seed.gen_range(config.npc_rand_speed_range.clone());

    let to_return = Pig {
        life_time: Timer::from_seconds(random_life_time, TimerMode::Once),
    };
    let speed = Speed(rand_speed);
    info!("Spawned pig: {:#?}", &to_return);
    return (
        to_return,
        speed,
        WanderingNpc::Wandering(rand_utils::create_rand_direction()),
        RandChangeWanderingNpc(Timer::from_seconds(rand_change_state, TimerMode::Repeating)),
    );
}
