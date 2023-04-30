use bevy::{prelude::*, window::PrimaryWindow};
use rust_rakitu_game::{GameState, PLANE_SIZE};


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

pub fn spawn_text(
    //window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    assert_server: Res<AssetServer>,
){
    //let window = window_query.get_single().unwrap();
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "hp: ",
                TextStyle {
                    font: assert_server.load("sprites/NotoSansKR-Regular.otf"),
                    font_size: 50.0,
                    color: Color::BLACK,
                },
            ),
            TextSection::from_style(TextStyle {
                font: assert_server.load("sprites/NotoSansKR-Regular.otf"),
                font_size: 50.0,
                color: Color::BLACK,
            }),
        ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(30.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                ..default()
            }),
    );
}

pub fn set_hp_score(
    mut text_query: Query<&mut Text>,
    gamestate: ResMut<GameState>,
){
    
    for (i, mut text) in text_query.iter_mut().enumerate(){
        if i == 0{
            text.sections[1].value = format!("{}", gamestate.hp);
        }else{
            text.sections[1].value = 0.to_string();
        }
    }
        
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