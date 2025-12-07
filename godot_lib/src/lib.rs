use std::net::SocketAddr;
use std::net::UdpSocket;
use std::time::Duration;
use std::time::SystemTime;

use bevy_renet::netcode::ClientAuthentication;
use bevy_renet::netcode::NetcodeClientTransport;
use bevy_renet::renet::Bytes;
use bevy_renet::renet::ConnectionConfig;
use bevy_renet::renet::DefaultChannel;
use bevy_renet::renet::RenetClient;
use client_message::Message;
use godot::classes::*;
use godot::prelude::*;

use protos::protos::messages::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Control)]
struct NetworkControl {
    base: Base<Control>,
    server_addr: Option<SocketAddr>,
    client: RenetClient,
    transport: Option<NetcodeClientTransport>,
}

#[godot_api]
impl IControl for NetworkControl {
    fn init(base: Base<Control>) -> Self {
        godot_print!("Network init");

        Self {
            base,
            server_addr: None, // If None, connection was not started
            client: RenetClient::new(ConnectionConfig::default()),
            transport: None,
        }
    }

    fn process(&mut self, delta: f64) {
        if let Some(transport) = self.transport.as_mut() {
            self.client.update(Duration::from_secs_f64(delta));
            transport
                .update(Duration::from_secs_f64(delta), &mut self.client)
                .unwrap();

            transport.send_packets(&mut self.client).unwrap();
        }

        if self.client.is_connected() {
            // Receive message from server
            while let Some(message) = self.client.receive_message(DefaultChannel::ReliableOrdered) {
                self.handle_server_message(message);
            }
        }
    }
}

#[godot_api]
impl NetworkControl {
    #[func]
    pub fn connect_to_host(&mut self, host: GString) {
        godot_print!("Connect to host");

        self.server_addr = Some(SocketAddr::V4(host.to_string().parse().unwrap()));
        let addr = self.server_addr.unwrap();

        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let authentication = ClientAuthentication::Unsecure {
            server_addr: addr,
            client_id: 0,
            user_data: None,
            protocol_id: 0,
        };

        self.transport =
            Some(NetcodeClientTransport::new(current_time, authentication, socket).unwrap());
    }

    #[func]
    pub fn send_debug_message(&mut self, message: GString) {
        godot_print!("Send debug {message}");

        let packet: ClientMessage = ClientMessage {
            message: Some(Message::DebugMessage(DebugMessage {
                content: String::from(message),
            })),
        };

        if self.server_addr.is_some() {
            self.client.send_message(
                DefaultChannel::ReliableOrdered,
                protos::serialize_client_message(packet),
            );
        }
    }

    fn handle_server_message(&mut self, &message: Bytes) {
        match protos::deserialize_server_message(&message) {
            Err(_) => println!("Could not deserialize message"),
            Ok(packet) => {
                if let Some(packet_message) = packet.message {
                    match packet_message {
                        server_message::Message::DebugMessage(debug_message) => {
                            godot_print!("[Server DEBUG]: {}", debug_message.content);
                        }
                    }
                }
            }
        }
    }
}
