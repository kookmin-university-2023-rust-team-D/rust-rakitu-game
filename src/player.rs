use bevy::{prelude::*, window::PrimaryWindow};

use crate::{PLAYER_SIZE, PLANE, PLAYER_SPEED, Player};

//플레이어 플러그인 생성
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(confine_player_movement);
    }
}

//플레이어 스폰 구현
pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assert_server: Res<AssetServer>,
){
    //window 객체 에서 윈도우 속성 뽑아오기
    let window: &Window = window_query.get_single().unwrap();

    //플레이어 엔티티 생성
    commands.spawn(
        (
            SpriteBundle{
                transform: Transform{
                    translation: Vec3::new(window.width() / 3.0, PLAYER_SIZE / 2.0 + PLANE, 0.0),
                    ..default()
                },
                    texture: assert_server.load("sprites/mario_running.png"),
                    ..default()
            },
            Player{},
        )
    );
}

//플레이어 움직임 구현
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform,With<Player>>,
    time: Res<Time>,
){
    //키보드 인풋을 받아 플레이어를 움직이게 만든다.
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

//플레이어 이동제한 구현
pub fn confine_player_movement(
    //Player 컴포넌트를 가진 엔티티를 가져온다.
    mut player_query: Query<&mut Transform,  With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,// With is used for using Player struct,

){
    //해당 엔티티를가 이동할시, 만약에 벽에 도달했을경우 그 이상으로 움직이지 못하도록 설정.
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
