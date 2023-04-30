use bevy::{prelude::*, window::PrimaryWindow};
use rust_rakitu_game::*;

//use bevy_matchbox::prelude::*;
use bevy_ggrs::*;

mod filewriter;
mod input;
mod setup;
mod p2p;
mod turtles;


use filewriter::*;
use input::*;
use setup::*;
use p2p::*;
use turtles::*;

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
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_plane)
    .add_startup_system(spawn_lakitu)
    .add_startup_systems((spawn_player, start_matchbox_socket))
    .add_startup_system(spawn_text)
    .add_systems((
        wait_for_players, 
        player_movement.in_schedule(GGRSSchedule),
        turtle_movement,
        turtle_hit_player,
        game_end_system,
        lakitu_movement,
        set_hp_score
    ))
    .run();   
}


//플레이어 움직임 구현
pub fn player_movement(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut player_query: Query<(&Player, &mut Transform), With<Rollback>>,
    gamestate: ResMut<GameState>,
    mut frame_count: ResMut<FrameCount>,
    window_query: Query<&Window, With<PrimaryWindow>>,
){  
    if !(gamestate.is_game_over){
        frame_count.frame += 1;
    }
    let window: &Window = window_query.get_single().unwrap(); 
    let x_min = 15.0;
    let x_max = window.width() - 15.0;
    for (player, mut transform) in player_query.iter_mut(){
        if !(gamestate.is_game_over){
            let mut direction = Vec2::ZERO;

        let (input, _) = inputs[player.handle];

        if input & INPUT_RIGHT != 0 {
            direction.x += 1.;
            transform.scale.x = 1.0;
        }
        if input & INPUT_LEFT != 0 {
            direction.x -= 1.;
            transform.scale.x = -1.0;
        }
        if input & INPUT_TURTLE != 0 && (frame_count.frame % 20 == 0){
            if player.is_enemy{
                let turtle_x = transform.translation.x;
                let turtle_y = transform.translation.y;
                println!("turtle spawn");
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(turtle_x, turtle_y, 0.0),
                        texture: assert_server.load("sprites/turtle.png"),
                        ..default()
                    },
                    Turtle{
                    },
                    Velocity{
                        speed: Vec3::new(0.0, -1.0, 0.0),
                    },
                ));
            }
        }
        if direction == Vec2::ZERO {
            continue;
        }
        println!("player {:?} moved", player.handle); 
        let move_speed = 30.0;
        let move_delta = (direction * move_speed).extend(0.);

        transform.translation += move_delta;
        let mut translation = transform.translation;
        if translation.x < x_min {
            translation.x = x_min;
        }
        else if translation.x > x_max {
            translation.x = x_max;
        }
        transform.translation = translation;
    }
}
}