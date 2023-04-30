use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::player::components::Player;
use bevy_ggrs::*;

use super::components::Turtle;

pub const ENEMY_SPEED: f32 = 300.0;
pub const PLAYER_SIZE: f32 = 70.0;
pub const TURTLE_SIZE: f32 = 60.0;

#[derive(Default, Reflect, Component)]
pub struct Velocity{
    pub speed: Vec3,
}


#[derive(Component)]
pub struct GameState{
    pub is_game_over: bool,
    pub score: i32,
}


// pub fn spawn_turtle(
//     mut commands: Commands,
//     //window_query: Query<&Window, With<PrimaryWindow>>,
//     asset_server: Res<AssetServer>,
//     mut enemy_query: Query<&mut Transform,  With<Player>>,
// ){
//     for transform in enemy_query.iter_mut(){
//         let turtle_x = transform.translation.x;
//         let turtle_y = transform.translation.y;

//         commands.spawn((
//             SpriteBundle {
//                 transform: Transform::from_xyz(turtle_x, turtle_y, 0.0),
//                 texture: asset_server.load("sprites/turtle.png"),
//                 ..default()
//             },
//             Turtle{
//             },
//             Velocity{
//                 speed: Vec3::new(0.0, -1.0, 0.0),
//             },
//         ));
//     }
// }

pub fn turtle_movement(
    mut commands: Commands,
    //window_query: Query<&Window, With<PrimaryWindow>>,
    mut turtle_query: Query<(Entity, &mut Velocity, &mut Transform),  With<Turtle>>,
    mut player_query: Query<&mut GameState, With<Player>>,
    time: Res<Time>,
){
    for (turtle, velocity, mut transform) in turtle_query.iter_mut(){
        let mut direction = Vec3::ZERO;
        //let window: &Window = window_query.get_single().unwrap(); 

        let y_min = 15.0;
        
        direction += velocity.speed;

        if direction.length() > 0.0{
            direction = direction.normalize();
        }

        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();

        let translation = transform.translation;
        if translation.y < y_min {
            for mut game_state in player_query.iter_mut(){
                game_state.score += 1;
            }
            commands.entity(turtle).despawn();
        }

        transform.translation = translation;
    }
    // let (mut velocity, mut transform) = enemy_query.single_mut();
    

}

pub fn turtle_hit_player(
    mut commands: Commands,
    //mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &mut Player, &Transform, &mut GameState), With<Rollback>>,
    enemy_query: Query<(Entity, &Transform), With<Turtle>>,
    //asset_server: Res<AssetServer>,
    //audio: Res<Audio>,
    //score: Res<Score>,
) {
    for (player_entity, mut player, player_transform, mut game_state) in  player_query.iter_mut() {
        for (turtle_entity, enemy_transform) in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = TURTLE_SIZE / 2.0;
            if distance < (player_radius + enemy_radius) && !(player.is_enemy) {
                println!("Enemy hit player!");
                //let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                //audio.play(sound_effect);
                commands.entity(turtle_entity).despawn();
                player.hp -= 1;
                if player.hp <= 0 {
                    game_state.is_game_over = true;
                    commands.entity(player_entity).despawn();
                    println!("Enemy hit player! Game Over!");
                    println!("Score: {}", game_state.score);
                }
                //game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}