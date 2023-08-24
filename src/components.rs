use bevy::{
    prelude::{Component, Vec2},
    reflect::Reflect,
    time::Timer,
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AabbColl;

#[derive(Component, Reflect)]
pub struct Speed(pub f32);

#[derive(Component)]
pub enum WanderingNpc {
    Idle,
    Wandering(Vec2),
}

#[derive(Component, Debug)]
pub struct Pig {
    pub life_time: Timer,
}

#[derive(Component)]
pub struct RandChangeWanderingNpc(pub Timer);

#[derive(Component)]
pub struct MainGameCamera;
#[derive(Component)]
pub struct MoneyScore;
