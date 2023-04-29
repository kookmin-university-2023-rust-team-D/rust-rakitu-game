use bevy::{prelude::*, window::PrimaryWindow};
use rust_rakitu_game::{player::PlayerPlugin, enemy::EnemyPlugin, PLANE_SIZE,PLANE, PLAYER_SIZE, TurtleSpawnTimer, turtles::TurtlePlugin};

use bevy_matchbox::prelude::*;
//use bevy::{prelude::*, render::camera::ScalingMode, tasks::IoTaskPool};
use bevy_ggrs::*;
//use matchbox_socket::{WebRtcSocket, PeerId};

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_FIRE: u8 = 1 << 4;



fn main() {
    
    // App::new();
    let mut app = App::new();
    GGRSPlugin::<GgrsConfig>::new()
        .with_input_system(input)
        .register_rollback_component::<Transform>()
        .build(&mut app);
    
    /*app.insert_resource(ClearColor(Color::WHITE))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Final Project Team B".to_string(),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }))*/
    app
    .add_plugins(DefaultPlugins)
    //.add_plugin(PlayerPlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(TurtlePlugin)
    .init_resource::<TurtleSpawnTimer>() // 기본적인 설정을 해줍니다. 이것만 있으면 검은색 공간이 appear
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_plane)
    .add_system(tick_turtle_spawn_timer)
    .add_startup_systems((spawn_player, start_matchbox_socket))
    .add_systems((wait_for_players, player_movement.in_schedule(GGRSSchedule)))
    .run();   
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
        input |= INPUT_FIRE;
    }

    input
}
pub struct GgrsConfig;

impl ggrs::Config for GgrsConfig {
    type Input = u8;
    type State = u8;
    type Address = PeerId;
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

    commands.insert_resource(bevy_ggrs::Session::P2PSession(ggrs_session));

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
    enemy_spawn_timer.spawn_timer.tick(time.delta());
}


pub const PLAYER_SPEED: f32 = 500.0;
#[derive(Component)]
pub struct Player{
    pub hp: i32,
    pub handle: usize
}

//플레이어 움직임 구현
pub fn player_movement(
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut player_query: Query<(&Player, &mut Transform),With<Player>>,
    //time: Res<Time>,
){
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
        if direction == Vec2::ZERO {
            continue;
        }

        println!("player {:?} moved", player.handle); 
        let move_speed = 30.0;
        let move_delta = (direction * move_speed).extend(0.);

        transform.translation += move_delta; 
    // //키보드 인풋을 받아 플레이어를 움직이게 만든다.
    // if let Ok(mut transform) = player_query.get_single_mut(){
    //     let mut direction = Vec3::ZERO;

    //     if input & INPUT_RIGHT != 0 {
    //         direction.x += 1.;
    //     }
    //     if input & INPUT_LEFT != 0 {
    //         direction.x -= 1.;
    //     }
    //     if direction == Vec2::ZERO {
    //         return;
    //     }
        // if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A){
        //     direction += Vec3::new(-1.0, 0.0, 0.0);
        //     transform.scale = Vec3::new(-1.0, 1.0, 0.0);
        // }
        // if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D){
        //     direction += Vec3::new(1.0, 0.0, 0.0);
        //     transform.scale = Vec3::new(1.0, 1.0, 0.0);
        // }

        // if direction.length() > 0.0{ 
        //     direction = direction.normalize();
        // }
        // transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
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
        )
    );

    //player 2
    //222222222
    commands.spawn(
        (
            Player{
                hp: 2,
                handle: 1
            },
            rip.next(),
            SpriteBundle{
                transform: Transform{
                    translation: Vec3::new(window.width() / 3.0, PLAYER_SIZE / 2.0 + PLANE, 0.0),
                    ..default()
                },
                    texture: assert_server.load("sprites/mario_stop.png"),
                    ..default()
            },
        )
    );
}

/*pub fn spawn(mut commands: Commands, mut rip: ResMut<RollbackIdProvider>) {
    commands.spawn(
        (
            Player { handle: 0 }, rip.next(), SpriteBundle {
        transform: Transform::from_translation(Vec3::new(-2., 0., 100.)),
        sprite: Sprite { color: Color::BLUE, ..default() },
        ..default()
    }));
    commands.spawn((Player { handle: 1 }, rip.next(), SpriteBundle {
        transform: Transform::from_translation(Vec3::new(2., 0., 100.)),
        sprite: Sprite { color: Color::RED, ..default() },
        ..default()
    }));
}*/
