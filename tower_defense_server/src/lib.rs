use std::net::UdpSocket;
use std::time::SystemTime;

use bevy::prelude::*;
use bevy_renet::RenetServerPlugin;
use bevy_renet::netcode::{
    NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerConfig,
};
use bevy_renet::renet::{ClientId, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};

use protos::protos::messages::*;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let server = RenetServer::new(ConnectionConfig::default());

        let server_addr = "0.0.0.0:5000".parse().unwrap();
        let socket = UdpSocket::bind(server_addr).unwrap();
        let server_config = ServerConfig {
            current_time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap(),
            max_clients: 64,
            protocol_id: 0,
            public_addresses: vec![server_addr],
            authentication: ServerAuthentication::Unsecure,
        };
        let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

        app.add_plugins(RenetServerPlugin)
            .insert_resource(server)
            .add_plugins(NetcodeServerPlugin)
            .insert_resource(transport)
            .add_systems(Update, (receive_message_system, handle_events_system));
    }
}

fn receive_message_system(mut server: ResMut<RenetServer>, mut _commands: Commands) {
    // Receive message from all clients
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            match protos::deserialize_client_message(&message) {
                Err(_) => println!("Could not deserialize message"),
                Ok(packet) => {
                    if let Some(packet_message) = packet.message {
                        match packet_message {
                            client_message::Message::DebugMessage(debug_message) => {
                                println!("Message from id:{client_id} {}", debug_message.content)
                            }
                        }
                    }
                }
            }
        }
    }
}

fn handle_events_system(
    mut server_events: MessageReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");
                send_debug_message(
                    &mut server,
                    *client_id,
                    format!("Welcome client {client_id}"),
                );
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}

fn send_debug_message(server: &mut ResMut<RenetServer>, client_id: ClientId, message: String) {
    let packet = ServerMessage {
        message: Some(server_message::Message::DebugMessage(DebugMessage {
            content: message,
        })),
    };
    server.send_message(
        client_id,
        DefaultChannel::ReliableOrdered,
        protos::serialize_server_message(packet),
    );
}
