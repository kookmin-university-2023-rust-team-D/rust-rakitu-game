use bevy::{prelude::*, window::PrimaryWindow};
use rust_rakitu_game::{player::PlayerPlugin, enemy::EnemyPlugin, Enemy, Velocity, PLANE_SIZE, ENEMY_SPEED};

pub const ENEMY_SPAWN_TIME: f32 = 2.0;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .add_plugin(EnemyPlugin)
    .init_resource::<EnemySpawnTimer>() // 기본적인 설정을 해줍니다. 이것만 있으면 검은색 공간이 appear
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_plane)
    .add_system(tick_enemy_spawn_timer)
    .add_system(spawn_enemies_over_time)
    .add_system(turtle_movement)
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



#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

#[derive(Component)]
pub struct Turtle{
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    //window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    mut enemy_query: Query<&mut Transform,  With<Enemy>>,
) {
    if enemy_spawn_timer.timer.finished() {
        //let window = window_query.get_single().unwrap();
        for (transform) in enemy_query.iter_mut(){
            let turtle_x = transform.translation.x;
            let turtle_y = transform.translation.y;

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(turtle_x, turtle_y, 0.0),
                    texture: asset_server.load("sprites/lakitu2.png"),
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
    for (turtle, mut velocity, mut transform) in turtle_query.iter_mut(){
        let mut direction = Vec3::ZERO;
        //let window: &Window = window_query.get_single().unwrap(); 

        let y_min = 15.0;
        
        direction += velocity.speed;

        if direction.length() > 0.0{
            direction = direction.normalize();
        }

        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();

        let mut translation = transform.translation;
        if translation.y < y_min {
            commands.entity(turtle).despawn();
        }

        transform.translation = translation;
    }
    // let (mut velocity, mut transform) = enemy_query.single_mut();
    

}

