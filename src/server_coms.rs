use std::net::TcpStream;
use websocket::{ClientBuilder, Message};
use websocket::sync::Client;
use yapping_core::l3gion_rust::StdError;
use yapping_core::serde::Serialize;
use yapping_core::server_message::ClientMessage;

#[derive(Default)]
pub struct ServerCommunication {
    connected: bool,
    server_ip: String,
    client: Option<Client<TcpStream>>,
}
impl ServerCommunication {
    pub fn connected(&self) -> bool {
        self.connected
    }

    pub fn try_connect(&mut self, ip: &str) -> Result<(), StdError> {
        let client = ClientBuilder::new(ip)?.connect_insecure()?;
        self.client = Some(client);
        self.connected = true;
        self.server_ip = ip.to_string();
        
        return Ok(());
    }

    pub fn send(&mut self, message: ClientMessage) -> Result<(), StdError> {
        if let Some(client) = &mut self.client {
            let message = Message::binary(yapping_core::bincode::serialize(&message)?);

            client.send_message(&message)?;
            
            return Ok(());
        }
        
        Err("Client is not connected to server! Cannot send message!".into())
    }
}