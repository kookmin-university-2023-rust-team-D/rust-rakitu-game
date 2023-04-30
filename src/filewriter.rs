use std::fs::File;
use std::io::prelude::*;
use bevy::{prelude::*};

use rust_rakitu_game::{GameState, FrameCount, PlayerIds};

pub fn write_file(gameover: &GameState, frame_count: &FrameCount, ids: &PlayerIds) -> std::io::Result<()> {
    let mut file = File::create("player_score.txt")?;

    let score = gameover.score;
    let frame = frame_count.frame / 60;
    let message = format!("player1's score: {}\nplayer1's survive time: {}", score, frame);
    let players = format!("\nplayer1: {}\nplayer2: {}", ids.player_ids[0], ids.player_ids[1]);
    file.write_all(message.as_bytes());
    file.write_all(players.as_bytes());

    Ok(())
}

pub fn game_end_system(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    gameover: Res<GameState>,
    input: Res<Input<KeyCode>>,
    ids: ResMut<PlayerIds>,
    frame_count: ResMut<FrameCount>
){
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if gameover.is_game_over && input.just_pressed(KeyCode::Q) {
            write_file(&gameover, &frame_count, &ids);
            commands.entity(window).despawn();
        }
        
    }
}