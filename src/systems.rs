use bevy::prelude::*;

use crate::{
    components::{
        MainGameCamera, MoneyScore, Pig, Player, RandChangeWanderingNpc, Speed, WanderingNpc,
    },
    pig_utils, rand_utils,
    resources::{GameConfig, Income, Money},
};

pub fn spawn_pig(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    config: Res<GameConfig>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_trans = player.single();
    let price = config.price;
    if money.0 >= price {
        money.0 -= price;
        info!("Spent ${} on a pig, remainnig money: ${}", money.0, price);
        pig_utils::create_pig(&player_trans, &mut commands, &assert_server, &config)
    } else {
        warn!("Not enought money require at least ${}", price)
    }
}

pub fn pig_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Speed, &WanderingNpc)>) {
    let secs = time.delta_seconds();
    for (mut pos, speed, wandering) in query.iter_mut() {
        if let WanderingNpc::Wandering(direction) = wandering {
            let movement = *direction * Vec2::splat(secs) * Vec2::splat(speed.0);
            let movement: Vec3 = (movement, 0.0).into();
            pos.translation += movement;
        }
    }
}

pub fn flip_wandering_npc(mut query: Query<(&mut Sprite, &WanderingNpc)>) {
    for (mut sprite, wandering) in query.iter_mut() {
        if let WanderingNpc::Wandering(direction) = wandering {
            sprite.flip_x = direction.x < 0.;
        }
    }
}

pub fn change_state_on_rand(
    mut query: Query<(&mut RandChangeWanderingNpc, &mut WanderingNpc)>,
    time: Res<Time>,
) {
    for (mut timer, mut state) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            *state = match *state {
                WanderingNpc::Idle => WanderingNpc::Wandering(rand_utils::create_rand_direction()),
                WanderingNpc::Wandering(_) => WanderingNpc::Idle,
            };
        }
    }
}

pub fn scale_player(mut query: Query<&mut Transform, With<Player>>, buttons: Res<Input<KeyCode>>) {
    if buttons.just_pressed(KeyCode::G) {
        let mut player = query.single_mut();
        player.scale += Vec3::new(0.1, 0.1, 0.0);
    }
}
pub fn keep_inside_window(
    camera_query: Query<&OrthographicProjection, With<MainGameCamera>>,
    mut coll_query: Query<(
        &mut Transform,
        &Sprite,
        &Handle<Image>,
        Option<&mut WanderingNpc>,
    )>,
    images: Res<Assets<Image>>,
) {
    for (mut trans, sprite, image, mut maybe_pig) in coll_query.iter_mut() {
        let x_y = Vec2::new(trans.translation.x, trans.translation.y);
        let (width, height) = if let Some(custom_area) = sprite.custom_size {
            (custom_area.x, custom_area.y)
        } else if let Some(image) = images.get(image) {
            let bounding_box = image;
            let bounding_box = bounding_box.texture_descriptor.size;
            (bounding_box.width as f32, bounding_box.height as f32)
        } else {
            info!("Not yet loaded");
            return;
        };
        let orth = camera_query.get_single().unwrap();
        let real_scale = Vec2::new(width, height) * trans.scale.truncate();

        let actual_area = Rect::from_center_size(x_y, real_scale);
        let (bound_left_x, bound_up_y, bound_right_x, actual_max_y) = corners(actual_area);
        let (min_x, min_y, max_x, max_y) = corners(orth.area);
        let tresspasser_x_y = &mut trans.translation;

        let x_resolved = resolve_outside_pos(&mut tresspasser_x_y.x, bound_right_x, max_x)
            || resolve_outside_neg(&mut tresspasser_x_y.x, bound_left_x, min_x);

        let y_resolved = resolve_outside_neg(&mut tresspasser_x_y.y, bound_up_y, min_y)
            || resolve_outside_pos(&mut tresspasser_x_y.y, actual_max_y, max_y);

        if x_resolved || y_resolved {
            if let Some(WanderingNpc::Wandering(direction)) = maybe_pig.as_deref_mut() {
                if x_resolved {
                    direction.x *= -1.;
                }
                if y_resolved {
                    direction.y *= -1.;
                }
            }
        }
    }

    fn corners(area: Rect) -> (f32, f32, f32, f32) {
        let Rect { min, max } = area;
        (min.x, min.y, max.x, max.y)
    }
    fn resolve_outside_pos(bound: &mut f32, actual: f32, max: f32) -> bool {
        if actual > max {
            let diff = (max - actual).abs();
            *bound -= diff;
            true
        } else {
            false
        }
    }
    fn resolve_outside_neg(bound: &mut f32, actual: f32, min: f32) -> bool {
        if actual < min {
            let diff = (min - actual).abs();
            *bound += diff;
            true
        } else {
            false
        }
    }
}
pub fn change_money_score(mut query: Query<&mut Text, With<MoneyScore>>, money: Res<Money>) {
    if money.is_changed() {
        let mut only_score = query.single_mut();
        only_score.sections[0].value = format!("Money: {}", money.0);
    }
}

pub fn handle_income(time: Res<Time>, mut money: ResMut<Money>, mut income: ResMut<Income>) {
    let timer = &mut income.when;
    timer.tick(time.delta());
    if timer.just_finished() {
        let amount = income.amount;
        money.0 += amount;
        info!(
            "Income increased by {}. Current amount: {}",
            amount, money.0
        );
    }
}

pub fn kill_pig(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Pig)>) {
    for (id, mut next) in query.iter_mut() {
        let timer = &mut next.life_time;
        timer.tick(time.delta());
        if timer.just_finished() {
            commands.entity(id).despawn_recursive();
        }
    }
}

pub fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite, &Speed), With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _, speed) in &mut characters {
        let delta = speed.0 * time.delta_seconds();
        if input.pressed(KeyCode::W) {
            transform.translation.y += delta;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= delta;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += delta;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= delta;
        }
    }
}
