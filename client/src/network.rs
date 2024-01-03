use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::prelude::*;
use bevy_renet::renet::{ConnectionConfig, RenetClient};
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError};
use bevy_renet::RenetClientPlugin;
use bevy_renet::transport::NetcodeClientPlugin;
use crate::game_state::{GameMode, GameState};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RenetClientPlugin)
            .add_plugins(NetcodeClientPlugin)
            .add_systems(Update, handle_events_system)

            .add_systems(OnEnter(GameMode::Multiplayer), on_enter_multiplayer_mode)
            .add_systems(OnExit(GameMode::Multiplayer), on_exit_multiplayer_mode);
    }
}

/// Вызывается при переходе в режим [GameMode::Multiplayer]
fn on_enter_multiplayer_mode(mut commands: Commands) {
    let client = RenetClient::new(ConnectionConfig::default());

    let authentication = ClientAuthentication::Unsecure {
        server_addr: "127.0.0.1:5000".parse().unwrap(),
        client_id: 0,
        user_data: None,
        protocol_id: 0,
    };
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    commands.insert_resource(transport);
    commands.insert_resource(client);
}

/// Вызывается при выходе из режима [GameMode::Multiplayer]
fn on_exit_multiplayer_mode(mut commands: Commands) {
    commands.remove_resource::<NetcodeClientTransport>();
    commands.remove_resource::<RenetClient>();
}

fn handle_events_system(
    mut renet_error: EventReader<NetcodeTransportError>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_mode: ResMut<NextState<GameMode>>,
) {
    // Слушаем ошибки со стороны сетевого слоя Renet и при обнаружении выбрасываем в главное меню
    for e in renet_error.read() {
        println!("{}", e);
        game_mode.set(GameMode::None);
        // TODO добавить сообщение об ошибке
        game_state.set(GameState::MainMenu);
    }
}