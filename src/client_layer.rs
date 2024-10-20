use yapping_core::l3gion_rust::lg_core::application::ApplicationCore;
use yapping_core::l3gion_rust::lg_core::event::LgEvent;
use yapping_core::l3gion_rust::lg_core::layer::Layer;
use yapping_core::l3gion_rust::sllog::info;
use yapping_core::l3gion_rust::{imgui, Rfc, StdError};

use crate::client_manager::ClientManager;
use crate::gui;
use crate::gui::theme::MAIN_THEME;
use crate::server_coms::ServerCommunication;

pub struct ClientLayer {
    app_core: ApplicationCore,
    server_coms: Rfc<ServerCommunication>,
    
    client_manager: ClientManager,
}
impl ClientLayer {
    pub fn new(app_core: ApplicationCore) -> Self {
        let server_coms = Rfc::new(ServerCommunication::default());
        // server_coms.borrow_mut().try_connect("ws://127.0.0.1:8080").unwrap();

        Self {
            app_core,
            server_coms: Rfc::clone(&server_coms),
            
            client_manager: ClientManager::new(
                server_coms,
                MAIN_THEME,
            ),
        }
    }
}

impl Layer for ClientLayer {
    fn debug_name(&self) -> &str {
        "ClientLayer"
    }

    fn on_attach(&mut self) -> Result<(), StdError> {
        info!("ClientLayer attached!");
        gui::init_gui(&mut self.app_core.renderer.borrow_mut())?;
        Ok(())
    }

    fn on_detach(&mut self) -> Result<(), StdError> {
        info!("ClientLayer detached!");
        Ok(())
    }

    fn on_update(&mut self) -> Result<(), StdError> {
        self.client_manager.on_update()
    }

    fn on_event(&mut self, event: &LgEvent) -> bool {
        match event {
            _ => ()
        }

        false
    }

    fn on_imgui(&mut self, ui: &mut imgui::Ui) {
        self.client_manager.on_imgui(ui, &self.app_core.renderer.borrow());
    }
}