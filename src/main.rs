use bevy::{prelude::*, window::PrimaryWindow};
use rust_rakitu_game::{player::PlayerPlugin, enemy::EnemyPlugin, PLANE_SIZE, TurtleSpawnTimer, turtles::TurtlePlugin};


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(TurtlePlugin)
    .init_resource::<TurtleSpawnTimer>() // 기본적인 설정을 해줍니다. 이것만 있으면 검은색 공간이 appear
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_plane)
    .add_system(tick_turtle_spawn_timer)
    .run();    
}

pub fn spawn_plane(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assert_server: Res<AssetServer>,
){
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn(
        (
            SpriteBundle{
                transform: Transform{
                    translation: Vec3::new(window.height() / 2.0 , 23.0, 0.0),
                    scale: PLANE_SIZE,
                    ..default()
                } ,// z component doesn't matter in 2D game
                texture: assert_server.load("sprites/tile_0002.png"),
                ..default()
            },
        )
    );
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,

){
    let window = window_query.get_single().unwrap();
    commands.spawn(
        Camera2dBundle{
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

pub fn tick_turtle_spawn_timer(mut enemy_spawn_timer: ResMut<TurtleSpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}