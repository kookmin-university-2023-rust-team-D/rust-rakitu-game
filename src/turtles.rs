use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ggrs::*;

use rust_rakitu_game::{Lakitu, Player, Turtle, FrameCount, GameState, Velocity, PLAYER_SIZE, TURTLE_SIZE, ENEMY_SPEED};

pub fn lakitu_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut enemy_query: Query<(&mut Velocity, &mut Transform),  With<Lakitu>>,
    time: Res<Time>,
){
    //Enemy 컴포넌트를 가진 엔티티들의 속도와 이동 설정
    for (mut velocity, mut transform) in enemy_query.iter_mut(){
        let mut direction: Vec3 = Vec3::ZERO;
        let window: &Window = window_query.get_single().unwrap(); 

        let x_min = 15.0;
        let x_max = window.width() - 15.0;
        direction += velocity.speed;

        if direction.length() > 0.0{
            direction = direction.normalize();
        }
        // 랜덤으로 생성된 속도 적용
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
        
        // 벽에 닿았을시, 밖으로 나갈 수 없에 설정
        let mut translation = transform.translation;
        if translation.x < x_min {
            translation.x = x_min;
            velocity.speed.x *= -1.0;
            transform.scale.x *= -1.0;
        }
        else if translation.x > x_max {
            translation.x = x_max;
            velocity.speed.x *= -1.0;
            transform.scale.x *= -1.0;
        }

        //최종 transform 설정
        transform.translation = translation;
    }
    // let (mut velocity, mut transform) = enemy_query.single_mut();
    

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