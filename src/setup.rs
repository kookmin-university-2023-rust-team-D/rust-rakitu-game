use bevy::{prelude::*, window::PrimaryWindow};
use rust_rakitu_game::{GameState, PLANE_SIZE, Lakitu, Velocity, ENEMY_SPEED, FrameCount};


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
                    top: Val::Px(50.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                ..default()
            }),
    );
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "time: ",
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
                    top: Val::Px(0.0),
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
    frame_count: ResMut<FrameCount>
){
    
    for (i, mut text) in text_query.iter_mut().enumerate(){
        if i == 0{
            text.sections[1].value = format!("{}", gamestate.hp);
        }else{
            text.sections[1].value = format!("{}", frame_count.frame/60);
        }
    }
        
}

pub fn spawn_lakitu(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
){
    let window = window_query.get_single().unwrap();
    commands.spawn(
        (
            Lakitu{
            },
            SpriteBundle{
                transform: Transform{
                    translation: Vec3::new(window.width() / 3.0, window.height() - 100.0, 0.0),
                    ..default()
                },
                    texture: assert_server.load("sprites/lakitu.png"),
                    ..default()
            },
            Velocity{
                speed: Vec3::new(1.0, 0.0, 0.0),
            },
        )
    );

}


pub fn lakitu_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut enemy_query: Query<(&mut Velocity, &mut Transform),  With<Lakitu>>,
    time: Res<Time>,
){
    //Enemy 컴포넌트를 가진 엔티티들의 속도와 이동 설정
    for (mut velocity, mut transform) in enemy_query.iter_mut(){
        let mut direction: Vec3 = Vec3::ZERO;
        let window: &Window = window_query.get_single().unwrap(); 

        let x_min = 15.0;
        let x_max = window.width() - 15.0;
        direction += velocity.speed;

        if direction.length() > 0.0{
            direction = direction.normalize();
        }
        // 랜덤으로 생성된 속도 적용
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
        
        // 벽에 닿았을시, 밖으로 나갈 수 없에 설정
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

        //최종 transform 설정
        transform.translation = translation;
    }
    // let (mut velocity, mut transform) = enemy_query.single_mut();
    

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