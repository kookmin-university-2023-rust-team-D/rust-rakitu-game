use bevy::{prelude::*, window::PrimaryWindow};


use crate::global::{PLAYER_SIZE, PLANE, PLAYER_SPEED};

#[derive(Component)]
pub struct Player{}

#[derive(Component)]
pub struct Velocity{
    pub speed: Vec3,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(confine_player_movement);
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assert_server: Res<AssetServer>,
){
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn(
        (
            SpriteBundle{
                transform: Transform{
                    translation: Vec3::new(window.width() / 3.0, PLAYER_SIZE / 2.0 + PLANE, 0.0),
                    ..default()
                },
                    texture: assert_server.load("sprites/mario_re.png"),
                    ..default()
            },
            Player{},
        )
    );
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

        if direction.length() > 0.0{
            direction = direction.normalize();
        }
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
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
