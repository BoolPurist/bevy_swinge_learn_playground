use bevy::prelude::*;
use rand::Rng;
pub fn create_rand_direction() -> Vec2 {
    let (x, y): (f32, f32) = (random_coord(), random_coord());
    return Vec2::new(x, y);

    fn random_coord() -> f32 {
        let mut rand_seed = rand::thread_rng();
        let coord: f32 = rand_seed.gen();
        if rand_seed.gen() {
            coord
        } else {
            -coord
        }
    }
}
