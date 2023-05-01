use bevy::{prelude::*};
use bevy_matchbox::prelude::*;
use bevy_ggrs::*;
use rust_rakitu_game::{GgrsConfig, PlayerIds, Cloud};

pub fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/room";
    info!("connecting to matchbox server: {:?}", room_url);
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

pub fn wait_for_players(
    mut commands: Commands, 
    mut socket: ResMut<MatchboxSocket<SingleChannel>>, 
    mut resource: ResMut<PlayerIds>, 
    mut cloud_query: Query<&mut Cloud>) 
    {
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

    for player in players.iter(){
        match player{
            ggrs::PlayerType::Local => {
                resource.player_ids.push("local".to_owned());
            },
            ggrs::PlayerType::Remote(peer_id) => {
                resource.player_ids.push(peer_id.0.to_string());
            }
            ggrs::PlayerType::Spectator(_) => {},
        }
    }

    for id in resource.player_ids.iter(){
        println!("{}",id);
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
    for mut cloud in cloud_query.iter_mut(){
        cloud.is_move = true;
    }

}