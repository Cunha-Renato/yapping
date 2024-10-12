use yapping_core::{l3gion_rust::{as_dyn, lg_core::{application::{ApplicationCreateInfo, L3gion}, layer::Layer, renderer::CreationWindowInfo}, Rfc}, server_message::ClientMessage};


mod client_layer;
mod server_coms;
mod gui;
mod client_manager;

fn main() {
    if cfg!(debug_assertions) {
        std::env::set_var("LOG", "4");
    }

    let mut l3gion = L3gion::new(ApplicationCreateInfo {
        window_info: CreationWindowInfo {
            event_loop: None,
            title: "Yapping".to_string(),
            width: 1080,
            height: 720,
            vsync: true,
        },
    }).unwrap();
    
    let l3gion_app = l3gion.get_app_mut();
    let client_layer = as_dyn!(
        client_layer::ClientLayer::new(l3gion_app.core()),
        dyn Layer
    );
    l3gion_app.push_layer(client_layer).unwrap();
    
    l3gion.run().unwrap();
}