use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ggrs::*;

use rust_rakitu_game::*;

pub fn cloud_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cloud_query: Query<(&mut Velocity, &mut Transform, &Cloud),  With<Cloud>>,
    time: Res<Time>,
){
    //Enemy 컴포넌트를 가진 엔티티들의 속도와 이동 설정
    for (mut velocity, mut transform, cloud) in cloud_query.iter_mut(){
        if !(cloud.is_move){
            continue;
        }
        let mut direction: Vec3 = Vec3::ZERO;
        let window: &Window = window_query.get_single().unwrap(); 

        let x_min = 30.0;
        let x_max = window.width() - 30.0;
        direction += velocity.speed;

        if direction.length() > 0.0{
            direction = direction.normalize();
        }
        // 랜덤으로 생성된 속도 적용
        transform.translation += direction * CLOUD_SPEED * time.delta_seconds();
        
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

//플레이어 움직임 구현
pub fn player_movement(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut player_query: Query<(&mut Player, &mut Transform), With<Rollback>>,
    gamestate: ResMut<GameState>,
    mut frame_count: ResMut<FrameCount>,
    window_query: Query<&Window, With<PrimaryWindow>>,
){  
    if !(gamestate.is_game_over){
        frame_count.frame += 1;
    }
    let window: &Window = window_query.get_single().unwrap(); 
    let x_min: f32 = 15.0;
    let x_max = window.width() - 15.0;
    for (mut player, mut transform) in player_query.iter_mut(){
        if !(gamestate.is_game_over){
            let mut direction = Vec2::ZERO;

        let (input, _) = inputs[player.handle];

        if input & INPUT_RIGHT != 0 {
            direction.x = 0.35;
            transform.scale.x = 1.0;
        }
        if input & INPUT_LEFT != 0 {
            direction.x = -0.35;
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
                        texture: assert_server.load("sprites/turtle_.png"),
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
             player.velocity = player.velocity * 0.97;
        }
        else{
            player.velocity = player.velocity + direction.x;
        }
        println!("player {:?} moved", player.handle); 
        // let move_speed = 30.0;
        // let move_delta = (direction * move_speed).extend(0.);
        let mut translation = transform.translation + Vec3::new(player.velocity/ 1.05, 0.0, 0.0);
        if translation.x < x_min {
            translation.x = x_min;
            player.velocity = 0.;
        }
        else if translation.x > x_max {
            translation.x = x_max;
            player.velocity = 0.;
        }
        transform.translation = translation;
    }
}
}