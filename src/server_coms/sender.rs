use std::sync::Arc;

use l3gion_rust::sllog::info;
use tokio::{io::AsyncWriteExt, net::TcpStream, runtime::Runtime, sync::Mutex};

#[derive(Clone)]
pub struct ServerSender {
    tcp_stream: Arc<Mutex<TcpStream>>,
}
impl ServerSender {
    pub async fn new() -> Result<Self, l3gion_rust::StdError> {
        let tcp_stream = Arc::new(Mutex::new(TcpStream::connect("127.0.0.1:8080").await.unwrap()));
        info!("Connection to server succeded!");
        
        Ok(Self {
            // tokio_runtime,
            tcp_stream,
        })
    }
    
    pub async fn send(&mut self, message: &str) {
        self.tcp_stream.lock().await.write_all(message.as_bytes()).await.unwrap();
    }
}