//enemy 구현 부분.
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use crate::{NUMBER_OF_ENEMIES, Enemy, Velocity, ENEMY_SPEED};

//에너미 플러그인 생성
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_enemy)
            .add_system(enemy_movement);
    }
}

// 에너미 스폰 구현
pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assert_server: Res<AssetServer>,
){
    let window: &Window = window_query.get_single().unwrap();
    //전역으로 설정된 NUMBER_OF_ENEMIES를 이용해서 적 생성
    for _ in 0..NUMBER_OF_ENEMIES{
        let random_x = random::<f32>() * window.width();

        // 엔티티 생성(SpriteBundle, Enemy, Velocity)
        commands.spawn(
            (
                SpriteBundle{
                    transform: Transform::from_xyz(random_x, window.height() - 100.0, 0.0),
                    texture: assert_server.load("sprites/lakitu.png"),
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
    //Enemy 컴포넌트를 가진 엔티티들의 속도와 이동 설정
    for (mut velocity, mut transform) in enemy_query.iter_mut(){
        let mut direction: Vec3 = Vec3::ZERO;
        let window: &Window = window_query.get_single().unwrap(); 

        let x_min = 15.0;
        let x_max = window.width() - 15.0;
        direction += velocity.speed;
        
        //랜덤 속도 설정
        let mut rng = rand::thread_rng();
        let rand_num = rng.gen_range(0..=250);
        if rand_num == 1{
            velocity.speed.x *= -1.0;
            transform.scale.x *= -1.0;
        }

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

