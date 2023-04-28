pub mod enemy;
pub mod player;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player{}

#[derive(Component)]
pub struct Enemy{
    pub level: f32,
}

#[derive(Component)]
pub struct Velocity{
    pub speed: Vec3,
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub spawn_timer: Timer,
    pub annoying_timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            spawn_timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
            annoying_timer: Timer::from_seconds(LAKITU_ANNOYING_TIME, TimerMode::Repeating),
        }
    }
}


#[derive(Component)]
pub struct Turtle{
}

pub const PLANE_X: f32 = 200.0;
pub const PLANE_SIZE: Vec3 = Vec3::new(PLANE_X, 3.0, 0.0);
pub const PLANE: f32 = 48.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 70.0;
pub const ENEMY_SPEED: f32 = 300.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPAWN_TIME: f32 = 2.0;
pub const LAKITU_ANNOYING_TIME: f32 = 5.0;