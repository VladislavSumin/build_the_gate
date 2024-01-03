use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::prelude::*;
use bevy_renet::renet::{ConnectionConfig, RenetServer, ServerEvent};
use bevy_renet::renet::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::RenetServerPlugin;
use bevy_renet::transport::NetcodeServerPlugin;

fn main() {
    let server = RenetServer::new(ConnectionConfig::default());

    // Transport layer setup
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(),
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();


    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(RenetServerPlugin)
        .add_plugins(NetcodeServerPlugin)
        .insert_resource(server)
        .insert_resource(transport)
        .add_systems(Update, handle_events_system)
        .run();
}

fn handle_events_system(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}