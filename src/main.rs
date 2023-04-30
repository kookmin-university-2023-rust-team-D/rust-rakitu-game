use bevy::{prelude::*, window::PrimaryWindow};
use rust_rakitu_game::{PLANE_SIZE, PLANE, PLAYER_SIZE};
use bevy::app::AppExit;
use bevy_matchbox::prelude::*;
//use bevy::{prelude::*, render::camera::ScalingMode, tasks::IoTaskPool};
use bevy_ggrs::*;
//use matchbox_socket::{WebRtcSocket, PeerId};

mod main_menu;
use main_menu::MainMenuPlugin;
 
mod game;
use game::GamePlugin;

use game::*;

pub mod events;
use events::*;


// use crate::events::GameOver;
// use crate::events::GameOver;

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_TURTLE: u8 = 1 << 4;
const INPUT_QUIT: u8 = 1 << 5;

pub const TURTLE_SIZE: f32 = 60.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 300.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const TURTLE_SPAWN_TIME: f32 = 0.1;
pub const FPS: usize = 60;


// #[derive(Reflect, Resource)]
// pub struct TurtleSpawnTimer {
//     pub spawn_timer: Timer,
// }
// impl Default for TurtleSpawnTimer {
//     fn default() -> TurtleSpawnTimer {
//         TurtleSpawnTimer {
//             spawn_timer: Timer::from_seconds(TURTLE_SPAWN_TIME, TimerMode::Repeating),
//         }
//     }
// }

#[derive(Resource, Default, Reflect, Hash)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}
fn main() {
    let mut app = App::new();
    GGRSPlugin::<GgrsConfig>::new()
        .with_input_system(input)
        .register_rollback_component::<Transform>()
        .with_update_frequency(FPS)
        .register_rollback_resource::<FrameCount>()
        .build(&mut app);
    
    app
    // MainMenu (added)
    .add_plugin(MainMenuPlugin)
    // Beby Plugins 
    .add_plugins(DefaultPlugins)
    .add_state::<AppState>()
    .insert_resource(FrameCount { frame: 0 })
    // Startup Systems
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_plane) 
    
    // .add_plugin(GamePlugin)
    .add_startup_systems((spawn_player, start_matchbox_socket))
    // // // Systems
    .add_systems((wait_for_players
        , player_movement.in_schedule(GGRSSchedule)))
    // .add_system(confine_player_movement)
    .add_system(turtle_movement)
    .insert_resource(Score{value:0})
    .add_system(turtle_hit_player)
    .add_system(game_end_system)
    .add_event::<GameOver>()
    .add_system(handle_game_over)
    .add_system(exit_game)
    .run();   
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState{
    #[default]
    MainMenu,
    Game,
    GameOver,
}
pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}
pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score.to_string());
        app_state_next_state.set(AppState::GameOver);
        println!("Entered AppState::GameOver");
    }
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

// pub fn tick_turtle_spawn_timer(mut enemy_spawn_timer: ResMut<TurtleSpawnTimer>, time: Res<Time>) {
//     enemy_spawn_timer.spawn_timer.tick(time.delta());
// }

#[derive(Component)]
pub struct Player{
    pub is_enemy: bool,
    pub hp: i32,
    pub handle: usize
}

#[derive(Component)]
pub struct Turtle{
}

#[derive(Default, Reflect, Component)]
pub struct Velocity{
    pub speed: Vec3,
}

#[derive(Component)]
pub struct GameState{
    pub is_game_over: bool,
    pub score: i32,
}

pub fn spawn_turtle(
    mut commands: Commands,
    //window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut enemy_query: Query<&mut Transform,  With<Player>>,
){
    for transform in enemy_query.iter_mut(){
        let turtle_x = transform.translation.x;
        let turtle_y = transform.translation.y;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(turtle_x, turtle_y, 0.0),
                texture: asset_server.load("sprites/turtle.png"),
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

pub fn turtle_movement(
    mut commands: Commands,
    //window_query: Query<&Window, With<PrimaryWindow>>,
    mut turtle_query: Query<(Entity, &mut Velocity, &mut Transform),  With<Turtle>>,
    mut player_query: Query<&mut GameState, With<Player>>,
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
            for mut game_state in player_query.iter_mut(){
                game_state.score += 1;
            }
            commands.entity(turtle).despawn();
        }

        transform.translation = translation;
    }
    // let (mut velocity, mut transform) = enemy_query.single_mut();
    

}


#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> HighScores {
        HighScores { scores: Vec::new() }
    }
}


pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.iter() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}


pub fn turtle_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &mut Player, &Transform, &mut GameState), With<Rollback>>,
    enemy_query: Query<(Entity, &Transform), With<Turtle>>,
    //asset_server: Res<AssetServer>,
    //audio: Res<Audio>,
    mut score: ResMut<Score>,
) {
    for (player_entity, mut player, player_transform, mut game_state) in  player_query.iter_mut() {
        for (turtle_entity, enemy_transform) in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = TURTLE_SIZE / 2.0;
            if distance < (player_radius + enemy_radius) && !(player.is_enemy) {
                println!("Enemy hit player!");
                //let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                //audio.play(sound_effect);
                commands.entity(turtle_entity).despawn();
                player.hp -= 1;
                // score.value += 1;
                if player.hp <= 0 {
                    game_state.is_game_over = true;
                    commands.entity(player_entity).despawn();
                    println!("Enemy hit player! Game Over!");
                    // println!("Score: {}", game_state.score);
                    game_over_event_writer.send(GameOver { score: score.value });

                }
            }
        }
    }
}

//플레이어 움직임 구현
pub fn player_movement(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut player_query: Query<(&Player, &mut Transform), With<Rollback>>,
    mut frame_count: ResMut<FrameCount>,
    //time: Res<Time>,
    mut score: ResMut<Score>,

){

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
                score.value += 1;
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

pub fn spawn_player(
    mut commands: Commands,
    mut rip: ResMut<RollbackIdProvider>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assert_server: Res<AssetServer>,
    mut score: ResMut<Score>,

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
