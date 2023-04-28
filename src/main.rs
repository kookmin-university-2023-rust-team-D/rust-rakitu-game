use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

const PLANE_X: f32 = 200.0;
const PLANE_SIZE: Vec3 = Vec3::new(PLANE_X, 3.0, 0.0);
const PLANE: f32 = 48.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 70.0;
pub const ENEMY_SPEED: f32 = 300.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPAWN_TIME: f32 = 2.0;
pub const LAKITU_ANNOYING_TIME: f32 = 5.0;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<EnemySpawnTimer>() // 기본적인 설정을 해줍니다. 이것만 있으면 검은색 공간이 appear
    .add_startup_system(spawn_player)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_plane)
    .add_startup_system(spawn_enemy)
    .add_system(player_movement)
    .add_system(enemy_movement)
    .add_system(tick_turtle_spawn_timer)
    .add_system(spawn_turtle_over_time)
    .add_system(turtle_movement)
    .add_system(tick_lakitu_annoying_timer)
    .add_system(lakitu_annoying_over_time)
    .add_system(confine_player_movement)
    .run();    
}
#[derive(Component)]
pub struct Player{}


#[derive(Component)]
pub struct Enemy{
    pub level: f32,
}

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

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assert_server: Res<AssetServer>,
){
    let window: &Window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_ENEMIES{
        let random_x = random::<f32>() * window.width();
        
        commands.spawn(
            (
                SpriteBundle{
                    transform: Transform::from_xyz(random_x, window.height() - 100.0, 0.0),
                    texture: assert_server.load("sprites/lakitu2.png"),
                    ..default()
                },
                Enemy{
                    level: 1.0,
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
    for (mut velocity, mut transform) in enemy_query.iter_mut(){
        let mut direction = Vec3::ZERO;
        let window: &Window = window_query.get_single().unwrap(); 

        let x_min = 15.0;
        let x_max = window.width() - 15.0;
        direction += velocity.speed;
        
        let mut rng = rand::thread_rng();
        let rand_num = rng.gen_range(0..=250);
        if rand_num == 1{
            velocity.speed.x *= -1.0;
            transform.scale.x *= -1.0;
        }

        if direction.length() > 0.0{
            direction = direction.normalize();
        }
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();

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

        transform.translation = translation;
    }
    // let (mut velocity, mut transform) = enemy_query.single_mut();
    

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


#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub spawn_timer: Timer,
    pub annoying_timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            spawn_timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
            annoying_timer: Timer::from_seconds(LAKITU_ANNOYING_TIME, TimerMode::Repeating),
        }
    }
}

pub fn tick_turtle_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.spawn_timer.tick(time.delta());
}

pub fn tick_lakitu_annoying_timer(mut lakitu_annoying_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    lakitu_annoying_timer.annoying_timer.tick(time.delta());
}

pub fn lakitu_annoying_over_time(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    mut enemy_query: Query<&mut Enemy,  With<Enemy>>,
) {
    if enemy_spawn_timer.annoying_timer.finished() {
        //let window = window_query.get_single().unwrap();
        for mut lakitu in enemy_query.iter_mut(){
            lakitu.level += 1.0;
            enemy_spawn_timer.spawn_timer = Timer::from_seconds(ENEMY_SPAWN_TIME/lakitu.level, TimerMode::Repeating);
            println!("{}", lakitu.level);
        }
    }
}


#[derive(Component)]
pub struct Turtle{
}

pub fn spawn_turtle_over_time(
    mut commands: Commands,
    //window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
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
}

pub fn turtle_movement(
    mut commands: Commands,
    //window_query: Query<&Window, With<PrimaryWindow>>,
    mut turtle_query: Query<(Entity, &mut Velocity, &mut Transform),  With<Turtle>>,
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


