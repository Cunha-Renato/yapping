use yapping_core::l3gion_rust::lg_core::application::ApplicationCore;
use yapping_core::l3gion_rust::lg_core::event::{KeyEvent, LgEvent, LgKeyCode};
use yapping_core::l3gion_rust::lg_core::layer::Layer;
use yapping_core::l3gion_rust::sllog::{error, info};
use yapping_core::l3gion_rust::{imgui, Rfc, StdError};

use crate::client_manager::ClientManager;
use crate::gui;
use crate::gui::theme::MAIN_THEME;
use crate::server_coms::ServerCommunication;

pub struct ClientLayer {
    app_core: ApplicationCore,
    server_coms: Rfc<ServerCommunication>,
    
    client_manager: ClientManager,
    show_debug_info: bool,
}
impl ClientLayer {
    pub(crate) fn new(app_core: ApplicationCore) -> Self {
        let server_coms = Rfc::new(ServerCommunication::new());
        let _ = server_coms.borrow_mut().try_connect("ws://127.0.0.1:8080");

        Self {
            app_core,
            server_coms: Rfc::clone(&server_coms),
            
            client_manager: ClientManager::new(
                server_coms,
                MAIN_THEME,
            ),
            show_debug_info: false,
        }
    }
}
// Private
impl ClientLayer {
    fn show_debug_info(&self, ui: &imgui::Ui) {
        ui.window("Debug Window")
            .size([450.0, 400.0], imgui::Condition::Appearing)
            .bg_alpha(1.0)
            .flags(imgui::WindowFlags::NO_TITLE_BAR)
            .build(|| {
                let _font = gui::use_font(ui, gui::FontType::REGULAR17);
                
                // Server coms
                ui.tree_node_config("ServerComs")
                    .framed(true)
                    .build(|| {
                        ui.text(std::format!("{}", &self.server_coms.borrow()));
                    });
                self.client_manager.show_debug_gui(ui);
            });
    }
}
impl Layer for ClientLayer {
    fn debug_name(&self) -> &str {
        "ClientLayer"
    }

    fn on_attach(&mut self) -> Result<(), StdError> {
        info!("ClientLayer attached!");
        gui::init_gui(&mut self.app_core.renderer.borrow_mut(), &self.app_core.window.borrow())?;
        self.client_manager.init()?;

        Ok(())
    }

    fn on_detach(&mut self) -> Result<(), StdError> {
        info!("ClientLayer detached!");
        self.client_manager.shutdown()?;

        Ok(())
    }

    fn on_update(&mut self) -> Result<(), StdError> {
        if let Err(e) = self.server_coms.borrow_mut().on_update() {
            error!("{e}");
        }
        
        let sent_responded = self.server_coms.borrow_mut().sent_responded();
        let received = self.server_coms.borrow_mut().received();

        self.client_manager.on_responded_messages(sent_responded)?;
        self.client_manager.on_received(received)?;

        Ok(())
    }

    fn on_event(&mut self, event: &LgEvent) -> bool {
        match event {
            LgEvent::KeyEvent(KeyEvent {  key: LgKeyCode::F1, pressed: true, ..}) => {
                self.show_debug_info = !self.show_debug_info;
                info!("{}", self.show_debug_info);
                    
                true
            },
            _ => false,
        }
    }

    fn on_imgui(&mut self, ui: &mut imgui::Ui) {
        self.client_manager.on_imgui(ui, &self.app_core.renderer.borrow());
        
        if self.show_debug_info {
            self.show_debug_info(ui);
        }
    }
}