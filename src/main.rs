use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

const PLANE_X: f32 = 200.0;
const PLANE_SIZE: Vec3 = Vec3::new(PLANE_X, 3.0, 0.0);
const PLANE: f32 = 48.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 70.0;
pub const OBJECT_SPEED: f32 = 300.0;
pub const NUMBER_OF_ENEMIES: usize = 1;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins) // 기본적인 설정을 해줍니다. 이것만 있으면 검은색 공간이 appear
    .add_startup_system(spawn_player)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_plane)
    .add_startup_system(spawn_enemy)
    .add_system(player_movement)
    .add_system(enemy_movement)
    .add_system(confine_player_movement)

    .run();    
}
#[derive(Component)]
pub struct Player{}


#[derive(Component)]
pub struct Enemy{}

#[derive(Component)]
pub struct Velocity{
    pub speed: Vec3,
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assert_server: Res<AssetServer>,
){
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn(
        (
            // SpriteBundle{
            //     transform: Transform::from_xyz(window.width() / 3.0, window.height() / 3.0, 0.0), // z component doesn't matter in 2D game
            //     texture: assert_server.load("sprites/Characters/character_0004.png"),
            //     ..default()
            // },
            SpriteBundle{
                transform: Transform{
                    translation: Vec3::new(window.width() / 3.0, PLAYER_SIZE / 2.0 + PLANE, 0.0),
                    scale: Vec3::new(3.0, 3.0, 0.0),
                    ..default()
                },
                    texture: assert_server.load("sprites/mario_re.png"),
                    ..default()
            },
            Player{},
        )
    );
}

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assert_server: Res<AssetServer>,
){
    let window: &Window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_ENEMIES{
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        
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
    mut object_query: Query<(&mut Velocity, &mut Transform),  With<Enemy>>,
    time: Res<Time>,
){
    let (mut velocity, mut transform) = object_query.single_mut();
    let mut direction = Vec3::ZERO;
    let window: &Window = window_query.get_single().unwrap(); 

    let x_min = 0.0;
    let x_max = window.width();
    direction += velocity.speed;

    if direction.length() > 0.0{
        direction = direction.normalize();
    }
    transform.translation += direction * OBJECT_SPEED * time.delta_seconds();

    let mut translation = transform.translation;
    if translation.x < x_min {
        translation.x = x_min;
        velocity.speed.x *= -1.0;
    }
    else if translation.x > x_max {
        translation.x = x_max;
        velocity.speed.x *= -1.0;
    }

    transform.translation = translation;

}



pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform,With<Player>>,
    time: Res<Time>,
){
    if let Ok(mut transform) = player_query.get_single_mut(){
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A){
            direction += Vec3::new(-1.0, 0.0, 0.0);
            transform.scale = Vec3::new(-1.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D){
            direction += Vec3::new(1.0, 0.0, 0.0);
            transform.scale = Vec3::new(1.0, 1.0, 0.0);
        }
        // if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W){
        //     direction += Vec3::new(0.0, 1.0, 0.0);
        // }
        // if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S){
        //     direction += Vec3::new(0.0, -1.0, 0.0);
        // }

        if direction.length() > 0.0{
            direction = direction.normalize();
        }
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
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
                texture: assert_server.load("sprites/Tiles/tile_0002.png"),
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

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform,  With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,// With is used for using Player struct,

){
    if let Ok(mut player_transform) = player_query.get_single_mut(){
        let window = window_query.get_single().unwrap();
        let half_player_size = PLAYER_SIZE / 2.0;

        let x_min = 0.0 + half_player_size;
        let x_max = window.width() -  half_player_size;
        let y_min = 0.0 + half_player_size + PLANE;
        let y_max = window.height() - half_player_size;
        let mut translation = player_transform.translation;

        if translation.x < x_min{
            translation.x = x_min;
        } else if translation.x > x_max{
            translation.x = x_max;
        }
        
        if translation.y < y_min{
            translation.y = y_min;
        } else if translation.y > y_max{
            translation.y = y_max;
        }
        player_transform.translation = translation;
    }
}
