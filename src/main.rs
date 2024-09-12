use l3gion_rust::{
    as_dyn, lg_core::{
    application::{
        ApplicationCreateInfo, 
        L3gion, 
        PersistentApplicationInfo
    }, layer::Layer, renderer::CreationWindowInfo
}, lg_types::reference::Rfc};

mod client_layer;
mod server_coms;

#[tokio::main]
async fn main() {
    std::env::set_var("LOG", "4");

    let mut l3gion = L3gion::new(ApplicationCreateInfo {
        persistant_info: PersistentApplicationInfo { v_sync: true },
        window_info: CreationWindowInfo {
            event_loop: None,
            title: "Yapping".to_string(),
            width: 1080,
            height: 720,
        },
    }).unwrap();
    
    let l3gion_app = l3gion.get_app_mut();
    let client_layer = as_dyn!(
        client_layer::ClientLayer::new(l3gion_app.core()).await,
        dyn Layer
    );
    l3gion_app.push_layer(client_layer).unwrap();
    
    l3gion.run().unwrap();
}