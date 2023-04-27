use bevy::{prelude::*, render::view::window, window::PrimaryWindow};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins) // 기본적인 설정을 해줍니다. 이것만 있으면 검은색 공간이 appear
    .add_startup_system(spawn_player)
    .add_startup_system(spawn_camera)
    .run();    
}

#[derive(Component)]
pub struct Player{}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assert_server: Res<AssetServer>,
){
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn(
        (
            SpriteBundle{
                transform: Transform::from_xyz(window.width() / 3.0, window.height() / 3.0, 0.0), // z component doesn't matter in 2D game
                texture: assert_server.load("sprites/Characters/character_0004.png"),
                ..default()
            },
            Player{},
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
            transform: Transform::from_xyz(window.height() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}
