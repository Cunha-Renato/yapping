use std::collections::VecDeque;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use websocket::{ClientBuilder, Message};
use websocket::sync::{Reader, Writer};
use yapping_core::l3gion_rust::sllog::error;
use yapping_core::l3gion_rust::StdError;
use yapping_core::server_message::{ClientMessage, ServerMessage};

#[derive(Default)]
pub(crate) struct ServerCommunication {
    connected: bool,
    server_ip: String,
    writer: Option<Writer<TcpStream>>,

    received_messages: Arc<Mutex<VecDeque<ServerMessage>>>,
}
impl ServerCommunication {
    pub(crate) fn on_update(&mut self) -> Result<(), StdError> {
        if !self.connected {
            self.try_connect(&self.server_ip.clone())?;
        }
        
        Ok(())
    }

    pub(crate) fn try_connect(&mut self, ip: &str) -> Result<(), StdError> {
        self.server_ip = ip.to_string();

        let (reader, writer) = ClientBuilder::new(ip)?.connect_insecure()?.split()?;
        self.start_reading(reader);
        self.writer = Some(writer);
        self.connected = true;
        
        return Ok(());
    }

    pub(crate) fn send(&mut self, message: ClientMessage) -> Result<(), StdError> {
        if let Some(writer) = &mut self.writer {
            let message = Message::binary(yapping_core::bincode::serialize(&message)?);

            match writer.send_message(&message) {
                Err(_) => {
                    writer.stream.shutdown(std::net::Shutdown::Both)
                        .map_err(|_| "Client is not connected to server! Cannot send message!")?;
                    
                    self.connected = false;
                    self.writer = None;
                },
                _ => (),
            }

            return Ok(());
        }
        
        Err("Client is not connected to server! Cannot send message!".into())
    }
}
impl ServerCommunication {
    fn start_reading(&self, mut reader: Reader<TcpStream>) {
        let messages = Arc::clone(&self.received_messages);

        let _ = std::thread::spawn(move || loop {
            match reader.recv_message() {
                Ok(msg) => match msg {
                    websocket::OwnedMessage::Binary(bin_msg) => {
                        let server_msg = yapping_core::bincode::deserialize::<ServerMessage>(&bin_msg).unwrap();
                        messages.lock().unwrap().push_back(server_msg);
                    },
                    _ => continue,
                },
                Err(e) => {
                    error!("{e}");
                    break;
                },
            }
        });
    }
}
impl Drop for ServerCommunication {
    fn drop(&mut self) {
        if let Some(writer) = &mut self.writer {
            let _ = writer.stream.shutdown(std::net::Shutdown::Both);
        }
    }
}