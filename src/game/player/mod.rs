use bevy::{prelude::*};

pub mod components;
pub mod systems;

use systems::*;

use bevy_ggrs::*;
// const INPUT_UP: u8 = 1 << 0;
// const INPUT_DOWN: u8 = 1 << 1;
// const INPUT_LEFT: u8 = 1 << 2;
// const INPUT_RIGHT: u8 = 1 << 3;
// const INPUT_TURTLE: u8 = 1 << 4;
// const INPUT_QUIT: u8 = 1 << 5;

// pub const TURTLE_SIZE: f32 = 60.0;
// pub const NUMBER_OF_ENEMIES: usize = 4;
// pub const ENEMY_SPEED: f32 = 300.0;
// pub const PLAYER_SPEED: f32 = 500.0;
// pub const TURTLE_SPAWN_TIME: f32 = 0.1;
// pub const FPS: usize = 60;
 


 
//플레이어 플러그인 생성
pub struct PlayerPlugin; 
 
impl Plugin for PlayerPlugin { 
    fn build(&self, app: &mut App) {
            app 
            // .add_system(player_movement)
            .add_systems((wait_for_players, player_movement.in_schedule(GGRSSchedule)))
            .add_startup_systems((spawn_player, start_matchbox_socket));
            // .add_system(confine_player_movement)
    }
}


