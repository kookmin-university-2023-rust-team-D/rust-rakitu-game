use bevy::prelude::*;

use crate::{Player, TurtleSpawnTimer, Enemy, Turtle, Velocity, ENEMY_SPEED, TURTLE_SIZE, PLAYER_SIZE};

pub struct TurtlePlugin;

impl Plugin for TurtlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_enemies_over_time)
            .add_system(turtle_movement)
            .add_system(turtle_hit_player);
    }
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    //window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<TurtleSpawnTimer>,
    mut enemy_query: Query<&mut Transform,  With<Enemy>>,
) {
    if enemy_spawn_timer.spawn_timer.finished() {
        //let window = window_query.get_single().unwrap();
        for transform in enemy_query.iter_mut(){
            let turtle_x = transform.translation.x;
            let turtle_y = transform.translation.y;

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(turtle_x, turtle_y, 0.0),
                    texture: asset_server.load("sprites/old_turtle.png"),
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
}

pub fn turtle_movement(
    mut commands: Commands,
    //window_query: Query<&Window, With<PrimaryWindow>>,
    mut turtle_query: Query<(Entity, &mut Velocity, &mut Transform),  With<Turtle>>,
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
            commands.entity(turtle).despawn();
        }

        transform.translation = translation;
    }
    // let (mut velocity, mut transform) = enemy_query.single_mut();
    

}

pub struct GameOver {
    pub score: u32,
}

pub fn turtle_hit_player(
    mut commands: Commands,
    //mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &mut Player, &Transform), With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Turtle>>,
    //asset_server: Res<AssetServer>,
    //audio: Res<Audio>,
    //score: Res<Score>,
) {
    if let Ok((player_entity, mut player, player_transform)) = player_query.get_single_mut() {
        for (turtle_entity, enemy_transform) in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = TURTLE_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player!");
                //let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                //audio.play(sound_effect);
                commands.entity(turtle_entity).despawn();
                player.hp -= 1;
                if player.hp <= 0 {
                    commands.entity(player_entity).despawn();
                    println!("Enemy hit player! Game Over!");
                }
                //game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}