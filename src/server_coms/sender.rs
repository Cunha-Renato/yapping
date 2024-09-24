use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use websocket::{ClientBuilder, Message};
use websocket::sync::Client;

use l3gion_rust::StdError;

#[derive(Default, Clone)]
pub struct ServerSender {
    connected: bool,
    server_ip: String,
    client: Option<Arc<Mutex<Client<TcpStream>>>>,
}
impl ServerSender {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn connected(&self) -> bool {
        self.connected
    }

    pub fn try_connect(&mut self, ip: &str) -> Result<(), StdError> {
        let client = ClientBuilder::new(ip)?.connect_insecure()?;
        self.client = Some(Arc::new(Mutex::new(client)));
        self.connected = true;
        self.server_ip = ip.to_string();
        
        return Ok(());
    }

    pub fn send(&mut self, message: &str) -> Result<(), StdError> {
        if let Some(client) = &mut self.client {
            let message = Message::text(message);

            client
                .lock()
                .unwrap()
                .send_message(&message)?;
            
            return Ok(());
        }
        
        Err("Client is not connected to server! Cannot send message!".into())
    }
}