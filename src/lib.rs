use bevy::{prelude::*};
use bevy_matchbox::prelude::*;

pub struct GgrsConfig;

impl ggrs::Config for GgrsConfig {
    type Input = u8;
    type State = u8;
    type Address = PeerId;
}

#[derive(Resource)]
pub struct GameState{
    pub is_game_over: bool,
    pub score: i32,
    pub hp: i32
}

#[derive(Resource)]
pub struct PlayerIds{
    pub player_ids: Vec<String>
}


#[derive(Resource, Default, Reflect, Hash)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}

#[derive(Component)]
pub struct Turtle{
}


#[derive(Component)]
pub struct Lakitu{
}


#[derive(Default, Reflect, Component)]
pub struct Velocity{
    pub speed: Vec3,
}

#[derive(Component)]
pub struct Player{
    pub is_enemy: bool,
    pub hp: i32,
    pub handle: usize,
    pub velocity: f32,
}

//사용된 전역 변수들
pub const PLANE_X: f32 = 200.0;
pub const PLANE_SIZE: Vec3 = Vec3::new(PLANE_X, 3.0, 0.0);
pub const PLANE: f32 = 48.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 70.0;
pub const TURTLE_SIZE: f32 = 60.0;
pub const ENEMY_SPEED: f32 = 300.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const TURTLE_SPAWN_TIME: f32 = 2.0;
pub const LAKITU_ANNOYING_TIME: f32 = 5.0;
pub const FPS: usize = 60;

pub const INPUT_UP: u8 = 1 << 0;
pub const INPUT_DOWN: u8 = 1 << 1;
pub const INPUT_LEFT: u8 = 1 << 2;
pub const INPUT_RIGHT: u8 = 1 << 3;
pub const INPUT_TURTLE: u8 = 1 << 4;