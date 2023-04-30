// pub mod enemy;
pub mod player;
pub mod turtles;

use bevy::prelude::*;
use crate::events::GameOver;


// use::enemy::EnemyPlugin;
use self::player::PlayerPlugin;
use self::turtles::TurtlePlugin;

 pub struct GamePlugin;

 impl Plugin for GamePlugin{
    fn build(&self, app: &mut App){
        app
            // Events
            // .add_event::<GameOver>()
            // Plugins
            // .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(TurtlePlugin);
    }
 }

