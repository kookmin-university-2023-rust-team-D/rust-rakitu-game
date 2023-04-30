use std::fs::File;
use std::io::prelude::*;
use bevy::{prelude::*};

#[derive(Resource)]
pub struct GameState{
    pub is_game_over: bool,
    pub score: i32,
    pub hp: i32
}


#[derive(Resource, Default, Reflect, Hash)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}



pub fn write_file(gameover: &GameState, frame_count: &FrameCount) -> std::io::Result<()> {
    let mut file = File::create("player_score.txt")?;

    let score = gameover.score;
    let frame = frame_count.frame / 60;
    let message = format!("score: {}\n time: {}", score, frame);
    file.write_all(message.as_bytes());

    Ok(())
}

pub fn game_end_system(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    gameover: Res<GameState>,
    input: Res<Input<KeyCode>>,
    frame_count: ResMut<FrameCount>
){
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if gameover.is_game_over && input.just_pressed(KeyCode::Q) {
            write_file(&gameover, &frame_count);
            commands.entity(window).despawn();
        }
        
    }
}