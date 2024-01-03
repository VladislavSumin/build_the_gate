use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::prelude::*;
use bevy_renet::renet::{ConnectionConfig, RenetClient};
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError};
use bevy_renet::RenetClientPlugin;
use bevy_renet::transport::NetcodeClientPlugin;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
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

        app
            .add_plugins(RenetClientPlugin)
            .add_plugins(NetcodeClientPlugin)
            .insert_resource(client)
            .insert_resource(transport)
            .add_systems(Update, handle_events_system);
    }
}

fn handle_events_system(mut renet_error: EventReader<NetcodeTransportError>) {
    for e in renet_error.read() {
        println!("{}", e);
    }
}