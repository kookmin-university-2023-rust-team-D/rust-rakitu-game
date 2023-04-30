pub fn Lakitu_movement(
    mut commands: Commands,
    mut lakitu_query: Query<(Entity, &mut Velocity, &mut Transform),  With<Lakitu>>,
    mut gamestate: ResMut<GameState>,
    time: Res<Time>,
){
    for (turtle, velocity, mut transform) in turtle_query.iter_mut(){
        let mut direction = Vec3::ZERO;

        let y_min = 15.0;
        
        direction += velocity.speed;

        if direction.length() > 0.0{
            direction = direction.normalize();
        }

        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();

        let translation = transform.translation;
        if translation.y < y_min {
            if !(gamestate.is_game_over){
                gamestate.score += 1;
            }
            commands.entity(turtle).despawn();
        }

        transform.translation = translation;
    }
}