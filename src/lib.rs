use bevy::{prelude::{Component, Vec3, Resource}, time::{Timer, TimerMode}};
// enemy, player 모듈화
// pub mod enemy;
// pub mod player;
// pub mod turtles;

// use bevy::prelude::{Component, Vec3};

#[derive(Component)]
pub struct Player{
    pub hp: i32,
}

#[derive(Component)]
pub struct Enemy{
    pub level: f32,
}

#[derive(Component)]
pub struct Velocity{
    pub speed: Vec3,
}
 
#[derive(Resource)]
pub struct TurtleSpawnTimer {
    pub spawn_timer: Timer,
    pub annoying_timer: Timer,
}

impl Default for TurtleSpawnTimer {
    fn default() -> TurtleSpawnTimer {
        TurtleSpawnTimer {
            spawn_timer: Timer::from_seconds(TURTLE_SPAWN_TIME, TimerMode::Repeating),
            annoying_timer: Timer::from_seconds(LAKITU_ANNOYING_TIME, TimerMode::Repeating),
        }
    }
}


#[derive(Component)]
pub struct Turtle{
}

// #[derive(Component)]
// pub struct Turtle{
// }

// #[derive(Resource)]
// pub struct TurtleSpawnTimer {
//     pub timer: Timer,
// }

// impl Default for TurtleSpawnTimer {
//     fn default() -> TurtleSpawnTimer {
//         TurtleSpawnTimer {
//             timer: Timer::from_seconds(ENEM_SPAWN_TIME, TimerMode::Repeating),
//         }
//     }
// }


//사용된 전역 변수들
pub const PLANE_X: f32 = 200.0;
pub const PLANE_SIZE: Vec3 = Vec3::new(PLANE_X, 3.0, 0.0);
pub const PLANE: f32 = 48.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 70.0;
pub const TURTLE_SIZE: f32 = 60.0;
pub const ENEMY_SPEED: f32 = 300.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const TURTLE_SPAWN_TIME: f32 = 0.5;
pub const LAKITU_ANNOYING_TIME: f32 = 5.0;