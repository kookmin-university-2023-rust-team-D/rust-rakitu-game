use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use crate::{NUMBER_OF_ENEMIES, Enemy, Velocity, ENEMY_SPEED};

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assert_server: Res<AssetServer>,
){
    let window: &Window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_ENEMIES{
        let random_x = random::<f32>() * window.width();
        
        commands.spawn(
            (
                SpriteBundle{
                    transform: Transform::from_xyz(random_x, window.height() - 100.0, 0.0),
                    texture: assert_server.load("sprites/lakitu2.png"),
                    ..default()
                },
                Enemy{},
                Velocity{
                    speed: Vec3::new(1.0, 0.0, 0.0)
                },
            )
        );
    }
}

pub fn enemy_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut enemy_query: Query<(&mut Velocity, &mut Transform),  With<Enemy>>,
    time: Res<Time>,
){
    for (mut velocity, mut transform) in enemy_query.iter_mut(){
        let mut direction: Vec3 = Vec3::ZERO;
        let window: &Window = window_query.get_single().unwrap(); 

        let x_min = 15.0;
        let x_max = window.width() - 15.0;
        direction += velocity.speed;
        
        let mut rng = rand::thread_rng();
        let rand_num = rng.gen_range(0..=250);
        if rand_num == 1{
            velocity.speed.x *= -1.0;
            transform.scale.x *= -1.0;
        }

        if direction.length() > 0.0{
            direction = direction.normalize();
        }
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();

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

        transform.translation = translation;
    }
    // let (mut velocity, mut transform) = enemy_query.single_mut();
    

}

