use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::mpsc::Receiver;
use websocket::{ClientBuilder, Message};
use websocket::sync::{Reader, Writer};
use yapping_core::l3gion_rust::lg_types::units_of_time::LgTime;
use yapping_core::l3gion_rust::sllog::error;
use yapping_core::l3gion_rust::{AsLgTime, LgTimer, StdError, UUID};
use yapping_core::server_message::{ClientMessage, ServerMessage, ServerMessageContent};

pub(crate) struct ServerCommunication {
    connected: bool,
    timer: LgTimer,
    server_ip: String,
    writer: Option<Writer<TcpStream>>,
    message_rx: Option<Receiver<ServerMessage>>,

    received_messages: HashMap<UUID, ServerMessageContent>,
}
impl ServerCommunication {
    pub(crate) fn new() -> Self {
        Self {
            connected: false,
            timer: LgTimer::new(), server_ip: String::default(),
            writer: None,
            message_rx: None,
            received_messages: HashMap::default(),
        }
    }

    pub(crate) fn connected(&self) -> bool {
        self.connected
    }

    fn is_connected(&mut self) -> bool {
        if let Some(writer) = &self.writer {
            self.connected = writer.stream.peer_addr().is_ok();
        }
        
        self.connected
    }
    
    pub(crate) fn on_update(&mut self) -> Result<(), StdError> {
        if self.timer.elapsed() >= 5_u32.s() {
            self.timer.restart();

            if !self.is_connected() {
                return self.try_connect(&self.server_ip.clone());
            }
        } 
        
        if let Some(rx) = &mut self.message_rx {
            while let Ok(msg) = rx.try_recv() {
                self.received_messages.insert(msg.uuid, msg.content);
            }
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

    pub(crate) fn send(&mut self, message: &ClientMessage) -> Result<(), StdError> {
        let writer = self.writer.as_mut().ok_or("Client is not connected to server! Cannot send message!")?;

        let message = Message::binary(yapping_core::bincode::serialize(&message)?);

        if writer.send_message(&message).is_err() {
            writer.stream.shutdown(std::net::Shutdown::Both).map_err(|_| "Client is not connected to server! Cannot send message!")?;
            
            self.writer = None;
            self.connected = false;
        }

        Ok(())
    }

    pub(crate) fn send_and_wait(&mut self, timeout: LgTime, message: &ClientMessage) -> Result<ServerMessageContent, StdError> {
        self.send(message)?;
        let rx = self.message_rx.as_mut().ok_or("Client is not connected to the Server!")?;

        let timer = LgTimer::new();
        while timer.elapsed() < timeout {
            if let Ok(msg) = rx.try_recv() {
                if msg.uuid == message.uuid {
                    return Ok(msg.content);
                }

                self.received_messages.insert(msg.uuid, msg.content);
            }
        }
        
        Ok(self.received_messages.remove(&message.uuid).ok_or("Response was not received!")?)
    }
}
impl ServerCommunication {
    fn start_reading(&mut self, mut reader: Reader<TcpStream>) {
        let (tx, rx) = std::sync::mpsc::channel();
        self.message_rx = Some(rx);

        let _ = std::thread::spawn(move || loop {
            match reader.recv_message() {
                Ok(msg) => match msg {
                    websocket::OwnedMessage::Binary(bin_msg) => {
                        let server_msg = yapping_core::bincode::deserialize::<ServerMessage>(&bin_msg).unwrap();
                        tx.send(server_msg).unwrap();
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