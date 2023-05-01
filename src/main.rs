use bevy::{prelude::*, window::PrimaryWindow};
use rust_rakitu_game::*;

//use bevy_matchbox::prelude::*;
use bevy_ggrs::*;

mod filewriter;
mod input;
mod setup;
mod p2p;
mod movement;


use filewriter::*;
use input::*;
use setup::*;
use p2p::*;
use movement::*;

fn main() {
    let mut app = App::new();
    GGRSPlugin::<GgrsConfig>::new()
        .with_input_system(input)
        .register_rollback_component::<Transform>()
        .with_update_frequency(FPS)
        .register_rollback_resource::<FrameCount>()
        .build(&mut app);
    
    app
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Final Project Team D".to_string(),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(FrameCount { frame: 0 })
    .insert_resource(GameState { is_game_over: false, score: 0, hp: 2})
    .insert_resource(PlayerIds { player_ids: Vec::new()})
    .insert_resource(ClearColor(Color::rgb(0.5, 0.6, 1.0)))
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_plane)
    .add_startup_systems((spawn_player, start_matchbox_socket))
    .add_startup_system(spawn_text)
    .add_systems((
        wait_for_players, 
        player_movement.in_schedule(GGRSSchedule),
        turtle_movement,
        turtle_hit_player,
        game_end_system,
        cloud_movement,
        set_hp_score
    ))
    .run();   
}