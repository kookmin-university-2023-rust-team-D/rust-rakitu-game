use bevy::prelude::*;


pub mod components;
pub mod systems;
 
use systems::*; 

// use crate::{Player, TurtleSpawnTimer, Enemy, Turtle, Velocity, ENEMY_SPEED, TURTLE_SIZE, PLAYER_SIZE};

pub struct TurtlePlugin;
impl Plugin for TurtlePlugin {
    fn build(&self, app: &mut App) {
         app 
            // .add_system(spawn_enemies_over_time)
            .add_system(turtle_movement)
            .add_system(turtle_hit_player); 
    }  
}

 