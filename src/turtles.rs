use bevy::prelude::*;

use crate::{TurtleSpawnTimer, Enemy, Turtle, Velocity, ENEMY_SPEED};

pub struct TurtlePlugin;

impl Plugin for TurtlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_enemies_over_time)
            .add_system(turtle_movement);
    }
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    //window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<TurtleSpawnTimer>,
    mut enemy_query: Query<&mut Transform,  With<Enemy>>,
) {
    if enemy_spawn_timer.timer.finished() {
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
