use std::fmt::Display;
use std::net::TcpStream;
use std::sync::mpsc::Receiver;
use websocket::{ClientBuilder, Message};
use websocket::sync::{Reader, Writer};
use yapping_core::l3gion_rust::lg_types::units_of_time::LgTime;
use yapping_core::l3gion_rust::sllog::{error, info, warn};
use yapping_core::l3gion_rust::{AsLgTime, LgTimer, StdError, UUID};
use yapping_core::client_server_coms::{ComsManager, Response, ServerMessage, ServerMessageContent};

pub(crate) struct ServerCommunication {
    connected: bool,
    timer: LgTimer,
    server_ip: String,
    writer: Option<Writer<TcpStream>>,
    message_rx: Option<Receiver<ServerMessage>>,
    
    manager: ComsManager,
}
impl ServerCommunication {
    pub(crate) fn new() -> Self {
        Self {
            connected: false,
            timer: LgTimer::new(), server_ip: String::default(),
            writer: None,
            message_rx: None,
            manager: ComsManager::default(),
        }
    }

    pub(crate) fn connected(&self) -> bool {
        self.connected
    }

    pub(crate) fn sent_responded(&mut self) -> Vec<(ServerMessage, Response)> {
        self.manager.sent_responded()
    }
    
    pub(crate) fn received(&mut self) -> Vec<ServerMessage> {
        self.manager.received_waiting()
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
                self.manager.received(msg);
            }
        }
        
        self.manager.update();
        for to_rety in self.manager.to_retry() {
            self.send(to_rety)?;
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

    pub(crate) fn send(&mut self, message: ServerMessage) -> Result<(), StdError> {
        warn!("Sent: {:?}", message);
        self.send_to_server(&message)?;
        self.manager.sent(message);

        Ok(())
    }

    pub(crate) fn send_and_wait(&mut self, timeout: LgTime, message: ServerMessage) -> Result<ServerMessageContent, StdError> {
        self.send_to_server(&message)?;

        let rx = self.message_rx.as_mut().ok_or("In ServerCommunication::send_and_wait: Client is not connected to the Server!")?;

        let timer = LgTimer::new();
        while timer.elapsed() < timeout {
            if let Ok(msg) = rx.try_recv() {
                if msg.uuid == message.uuid {
                    return Ok(msg.content);
                }

                self.manager.sent(msg);
            }
        }
        
        Err("In ServerCommunication::send_and_wait: Timeout!".into())
    }
    
    pub(crate) fn wait_for_response(&mut self, msg_uuid: UUID) {
        while self.manager.was_responded(msg_uuid) != true {}
    }
}
// Private
impl ServerCommunication {
    fn is_connected(&mut self) -> bool {
        if let Some(writer) = &self.writer {
            self.connected = writer.stream.peer_addr().is_ok();
        }
        
        self.connected
    }
    
    fn send_to_server(&mut self, message: &ServerMessage) -> Result<(), StdError> {
        let writer = self.writer.as_mut().ok_or("Client is not connected to server! Cannot send message!")?;
        let bin_message = Message::binary(yapping_core::bincode::serialize(&message)?);

        if writer.send_message(&bin_message).is_err() {
            writer.stream.shutdown(std::net::Shutdown::Both).map_err(|_| "Client is not connected to server! Cannot send message!")?;
            
            self.writer = None;
            self.connected = false;
        }
        
        Ok(())
    }

    fn start_reading(&mut self, mut reader: Reader<TcpStream>) {
        let (tx, rx) = std::sync::mpsc::channel();
        self.message_rx = Some(rx);

        let _ = std::thread::spawn(move || loop {
            match reader.recv_message() {
                Ok(msg) => match msg {
                    websocket::OwnedMessage::Binary(bin_msg) => {
                        let server_msg = yapping_core::bincode::deserialize::<ServerMessage>(&bin_msg).unwrap();
                        info!("Received: {:?}", server_msg);
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
impl Display for ServerCommunication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServerCommunication:\n\tconnected: {}\n\tmanager: {:#?}", self.connected, self.manager)
    }
}
impl Drop for ServerCommunication {
    fn drop(&mut self) {
        if let Some(writer) = &mut self.writer {
            let _ = writer.stream.shutdown(std::net::Shutdown::Both);
        }
    }
}