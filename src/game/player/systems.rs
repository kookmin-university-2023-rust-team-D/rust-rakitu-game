use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;


use crate::game::turtles::components::Turtle;

use super::components::Player;
 

// input
const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_TURTLE: u8 = 1 << 4;
const INPUT_QUIT: u8 = 1 << 5;
 

// another variable
pub const PLAYER_SIZE: f32 = 70.0;
pub const PLANE: f32 = 48.0;
 pub const FPS: usize = 60;

pub struct GgrsConfig;

impl ggrs::Config for GgrsConfig {
    type Input = u8;
    type State = u8;
    type Address = PeerId;
}
#[derive(Default, Reflect, Component)]
pub struct Velocity{
    pub speed: Vec3,
}

#[derive(Resource, Default, Reflect, Hash)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}

#[derive(Component)]
pub struct GameState{
    pub is_game_over: bool,
    pub score: i32,
}



fn input(_: In<ggrs::PlayerHandle>, keys: Res<Input<KeyCode>>) -> u8 {
    let mut input = 0u8;

    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        input |= INPUT_UP;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        input |= INPUT_DOWN;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        input |= INPUT_LEFT
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        input |= INPUT_RIGHT;
    }
    if keys.any_pressed([KeyCode::Space, KeyCode::Return]) {
        input |= INPUT_TURTLE;
    }
    if keys.any_pressed([KeyCode::Q]){
        input |= INPUT_QUIT;
    }

    input
}

pub fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/room";
    info!("connecting to matchbox server: {:?}", room_url);
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

pub fn wait_for_players(mut commands: Commands, mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");
    // TODO
    // create a GGRS P2P session
    let mut session_builder = ggrs::SessionBuilder::<GgrsConfig>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    // move the channel out of the socket (required because GGRS takes ownership of it)
    let channel = socket.take_channel(0).unwrap();

    // start the GGRS session
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");
    println!("im in!!!!!!!!!! wait!!!!");
    commands.insert_resource(bevy_ggrs::Session::P2PSession(ggrs_session));
    println!("im in!!!!!!!!!! wait!!!!");

}
pub fn spawn_player(
    mut commands: Commands,
    mut rip: ResMut<RollbackIdProvider>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assert_server: Res<AssetServer>,
){
    //window 객체 에서 윈도우 속성 뽑아오기
    let window: &Window = window_query.get_single().unwrap();

    //플레이어1 엔티티 생성
    //111111
    commands.spawn(
        (
            Player{
                is_enemy: false,
                hp: 2,
                handle: 0
            },
            rip.next(),
            SpriteBundle{
                transform: Transform{
                    translation: Vec3::new(window.width() / 3.0, PLAYER_SIZE / 2.0 + PLANE, 0.0),
                    ..default()
                },
                    texture: assert_server.load("sprites/mario_running.png"),
                    ..default()
            },
            GameState{
                is_game_over: false,
                score: 0
            }
        )
    );

    //player 2
    //222222222
    commands.spawn(
        (
            Player{
                is_enemy: true,
                hp: 2,
                handle: 1
            },
            rip.next(),
            SpriteBundle{
                transform: Transform{
                    translation: Vec3::new(window.width() / 3.0, window.height() - 100.0, 0.0),
                    ..default()
                },
                    texture: assert_server.load("sprites/lakitu.png"),
                    ..default()
            },
            GameState{
                is_game_over: false,
                score: 0
            }
        )
    );
}

pub fn game_end_system(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<Input<KeyCode>>,
){
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }
        if input.just_pressed(KeyCode::Q) {
            commands.entity(window).despawn();
        }
    }
}

//플레이어 움직임 구현
//플레이어 움직임 구현
pub fn player_movement(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut player_query: Query<(&Player, &mut Transform), With<Rollback>>,
    mut frame_count: ResMut<FrameCount>,
    //time: Res<Time>,
){
    println!("im in!!!!!!!!!! move!!!!");

    frame_count.frame += 1;
    for (player, mut transform) in player_query.iter_mut(){ 
        let mut direction = Vec2::ZERO;

        let (input, _) = inputs[player.handle];

        if input & INPUT_UP != 0 {
            direction.y += 1.;
        }
        if input & INPUT_DOWN != 0 {
            direction.y -= 1.;
        }
        if input & INPUT_RIGHT != 0 {
            direction.x += 1.;
        }
        if input & INPUT_LEFT != 0 {
            direction.x -= 1.;
        }
        if input & INPUT_TURTLE != 0 && (frame_count.frame % 20 == 0){
            if player.is_enemy{
                let turtle_x = transform.translation.x;
                let turtle_y = transform.translation.y;
                println!("turtle spawn");
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(turtle_x, turtle_y, 0.0),
                        texture: assert_server.load("sprites/turtle.png"),
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
            continue;
        }

        println!("player {:?} moved", player.handle); 
        let move_speed = 30.0;
        let move_delta = (direction * move_speed).extend(0.);

        transform.translation += move_delta; 
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