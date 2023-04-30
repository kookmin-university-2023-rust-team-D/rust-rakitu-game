//enemy 구현 부분.
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use crate::{NUMBER_OF_ENEMIES, Enemy, Velocity, ENEMY_SPEED, TURTLE_SPAWN_TIME, TurtleSpawnTimer, Turtle};

//에너미 플러그인 생성
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_enemy)
            .add_system(enemy_movement)
            .add_system(tick_lakitu_annoying_timer)
            .add_system(lakitu_annoying_over_time);
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
                    texture: assert_server.load("sprites/old-lakitu.png"),
                    ..default()
                },
                Enemy{
                    level: 1.0
                },
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

pub fn tick_turtle_spawn_timer(mut enemy_spawn_timer: ResMut<TurtleSpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.spawn_timer.tick(time.delta());
}

pub fn tick_lakitu_annoying_timer(mut lakitu_annoying_timer: ResMut<TurtleSpawnTimer>, time: Res<Time>) {
    lakitu_annoying_timer.annoying_timer.tick(time.delta());
}

pub fn lakitu_annoying_over_time(
    mut enemy_spawn_timer: ResMut<TurtleSpawnTimer>,
    mut enemy_query: Query<&mut Enemy,  With<Enemy>>,
) {
    if enemy_spawn_timer.annoying_timer.finished() {
        //let window = window_query.get_single().unwrap();
        for mut lakitu in enemy_query.iter_mut(){
            lakitu.level += 1.0;
            enemy_spawn_timer.spawn_timer = Timer::from_seconds(TURTLE_SPAWN_TIME/lakitu.level, TimerMode::Repeating);
            println!("{}", lakitu.level);
        }
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
                    texture: asset_server.load("sprites/old-lakitu.png"),
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
    mut turtle_query: Query<(Entity, &Velocity, &mut Transform),  With<Turtle>>,
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

