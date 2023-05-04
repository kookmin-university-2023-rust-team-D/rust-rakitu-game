use bevy::{prelude::*, window::PrimaryWindow};
use rust_rakitu_game::*;
//use bevy_matchbox::prelude::*;
use bevy_ggrs::*;

use std::env;

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
    let args: Vec<String> = env::args().collect();

    // 벡터에서 두 번째 인자를 가져와 변수에 저장합니다.
    let mut second_arg = args.get(1).unwrap_or(&String::new()).clone();

    // 두 번째 인자가 존재하는 경우에만 출력합니다.
    if !second_arg.is_empty() {
        println!("두 번째 인자: {}", second_arg);
    } else {
        println!("두 번째 인자가 존재하지 않습니다.");
        second_arg = "127.0.0.1".to_string();
    }


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
    .insert_resource(Server{address: second_arg})
    .insert_resource(FrameCount { frame: 0 })
    .insert_resource(GameState { is_game_over: false, score: 0, hp: 30})
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