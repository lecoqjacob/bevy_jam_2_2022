use crate::{prelude::*, GGRSConfig};
use bevy::tasks::IoTaskPool;
use bevy_ggrs::SessionType;
use ggrs::{PlayerHandle, PlayerType, SessionBuilder};
use matchbox_socket::WebRtcSocket;

mod connect_ui;
use connect_ui::*;

pub struct LocalHandles {
    pub handles: Vec<PlayerHandle>,
}

pub struct ConnectData {
    pub lobby_id: String,
}

//const MATCHBOX_ADDR: &str = "ws://127.0.0.1:3536";
const MATCHBOX_ADDR: &str = "wss://match.gschup.dev";

pub fn create_matchbox_socket(mut commands: Commands, connect_data: Res<ConnectData>) {
    println!("create_matchbox_socket {}", MATCHBOX_ADDR);

    let lobby_id = &connect_data.lobby_id;
    let room_url = format!("{MATCHBOX_ADDR}/{lobby_id}");
    let (socket, message_loop) = WebRtcSocket::new(room_url);
    IoTaskPool::get().spawn(message_loop).detach();
    commands.insert_resource(Some(socket));
    commands.remove_resource::<ConnectData>();
}

pub fn update_matchbox_socket(mut commands: Commands, mut socket_res: ResMut<Option<WebRtcSocket>>) {
    println!("update_matchbox_socket");
    if let Some(socket) = socket_res.as_mut() {
        socket.accept_new_connections();

        println!("update_matchbox_socket..: {:?}", socket.players().len());
        if socket.players().len() >= NUM_PLAYERS {
            // take the socket
            let socket = socket_res.as_mut().take().unwrap();
            create_ggrs_session(&mut commands, socket);
            commands.insert_resource(NextState(AppState::RoundOnline));
        }
    }
}

fn create_ggrs_session(commands: &mut Commands, socket: WebRtcSocket) {
    // create a new ggrs session
    let mut sess_build = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(NUM_PLAYERS)
        .with_max_prediction_window(MAX_PREDICTION)
        .with_fps(FPS)
        .expect("Invalid FPS")
        .with_input_delay(INPUT_DELAY);

    // add players
    let mut handles = Vec::new();
    for (i, player_type) in socket.players().iter().enumerate() {
        if *player_type == PlayerType::Local {
            handles.push(i);
        }
        sess_build = sess_build.add_player(player_type.clone(), i).expect("Invalid player added.");
    }

    // start the GGRS session
    let sess = sess_build.start_p2p_session(socket).expect("Session could not be created.");

    commands.insert_resource(sess);
    commands.insert_resource(LocalHandles { handles });
    commands.insert_resource(SessionType::P2PSession);
}

///////////////////////////////////////////////////////////////////////////////
pub struct ConnectPlugin;
impl Plugin for ConnectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ConnectUIPlugin);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::MenuConnect)
                .with_system(update_matchbox_socket.run_if_resource_exists::<Option<WebRtcSocket>>())
                .into(),
        );
    }
}
///////////////////////////////////////////////////////////////////////////////
