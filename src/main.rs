use bevy::{prelude::*, window::PrimaryWindow};
use rust_rakitu_game::*;

//use bevy_matchbox::prelude::*;
use bevy_ggrs::*;

mod filewriter;
mod input;
mod setup;
mod p2p;

use filewriter::*;
use input::*;
use setup::*;
use p2p::*;

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

pub fn turtle_movement(
    mut commands: Commands,
    mut turtle_query: Query<(Entity, &mut Velocity, &mut Transform),  With<Turtle>>,
    mut gamestate: ResMut<GameState>,
    time: Res<Time>,
){
    for (turtle, velocity, mut transform) in turtle_query.iter_mut(){
        let mut direction = Vec3::ZERO;

        let y_min = 15.0;
        
        direction += velocity.speed;

        if direction.length() > 0.0{
            direction = direction.normalize();
        }

        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();

        let translation = transform.translation;
        if translation.y < y_min {
            if !(gamestate.is_game_over){
                gamestate.score += 1;
            }
            commands.entity(turtle).despawn();
        }

        transform.translation = translation;
    }
}

pub fn turtle_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Player, &Transform), With<Rollback>>,
    mut gamestate: ResMut<GameState>,
    enemy_query: Query<(Entity, &Transform), With<Turtle>>,
    frame_count: ResMut<FrameCount>
) {
    for (player_entity, player, player_transform) in  player_query.iter_mut() {
        for (turtle_entity, enemy_transform) in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = TURTLE_SIZE / 2.0;
            if distance < (player_radius + enemy_radius) && !(player.is_enemy) {
                println!("Enemy hit player!");
                commands.entity(turtle_entity).despawn();
                gamestate.hp -= 1;
                if gamestate.hp <= 0 {
                    gamestate.is_game_over = true;
                    println!("{}", gamestate.is_game_over);
                    if !(player.is_enemy){
                        commands.entity(player_entity).despawn();
                    }
                    println!("Enemy hit player! Game Over!");
                    println!("Score: {}", gamestate.score);
                    println!("Time: {}", frame_count.frame / 60);
                }
                //game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
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

pub fn spawn_player(
    mut commands: Commands,
    mut rip: ResMut<RollbackIdProvider>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assert_server: Res<AssetServer>,
){
    //window 객체 에서 윈도우 속성 뽑아오기
    let window: &Window = window_query.get_single().unwrap();

    //플레이어1 엔티티 생성
    //111111
    println!("player spawn");
    commands.spawn(
        (
            Player{
                is_enemy: false,
                hp: 2,
                handle: 0,
            },
            rip.next(),
            SpriteBundle{
                transform: Transform{
                    translation: Vec3::new(window.width() / 3.0, PLAYER_SIZE / 2.0 + PLANE, 0.0),
                    ..default()
                },
                    texture: assert_server.load("sprites/mario_running.png"),
                    ..default()
            },
        )
    );
    //player 2
    //222222222
    commands.spawn(
        (
            Player{
                is_enemy: true,
                hp: 2,
                handle: 1,
            },
            rip.next(),
            SpriteBundle{
                transform: Transform{
                    translation: Vec3::new(window.width() / 3.0, window.height() - 100.0, 0.0),
                    ..default()
                },
                    texture: assert_server.load("sprites/lakitu.png"),
                    ..default()
            },
        )
    );
}