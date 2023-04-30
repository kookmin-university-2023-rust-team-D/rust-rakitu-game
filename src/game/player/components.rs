use bevy::prelude::*;
#[derive(Component)]
pub struct Player{
    pub is_enemy: bool,
    pub hp: i32,
    pub handle: usize
}

// #[derive(Component)]
// pub struct GameState{
//     pub is_game_over: bool,
//     pub score: i32,
// }

