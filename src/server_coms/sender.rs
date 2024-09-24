use std::{io::Write, net::TcpStream, sync::{Arc, Mutex}};

use l3gion_rust::{sllog::info, StdError};

#[derive(Default, Clone)]
pub struct ServerSender {
    connected: bool,
    server_ip: String,
    tcp_stream: Option<Arc<Mutex<TcpStream>>>,
}
impl ServerSender {
    pub async fn new() -> Self {
        Self::default()
    }
    
    pub fn connected(&self) -> bool {
        self.connected
    }

    pub fn try_connect(&mut self, ip: &str) -> Result<(), StdError> {
        match TcpStream::connect(ip) {
            Ok(tcp_stream) => {
                self.tcp_stream = Some(Arc::new(Mutex::new(tcp_stream)));
                self.connected = true;
                self.server_ip = ip.to_string();
                
                info!("Connected: {} on: {}", self.connected, self.server_ip);
                
                Ok(())
            }
            Err(e) => {
                Err(Box::new(e))
            }
        }
        
    }

    pub fn send(&mut self, message: &str) -> Result<(), StdError> {
        if let Some(tcp_stream) = &self.tcp_stream {
            return match tcp_stream.lock().unwrap().write_all(message.as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            };
        }
        
        Err("Client is not connected to server! Cannot send message!".into())
    }
}