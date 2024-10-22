use yapping_core::l3gion_rust::{imgui, lg_core::renderer::Renderer, sllog::info, Rfc, StdError};
use crate::{gui::{theme::Theme, validation_gui::validation_gui_manager::ValidationGuiManager}, server_coms::ServerCommunication, ClientMessage};

#[allow(non_camel_case_types)]
pub(crate) enum ForegroundState {
    MAIN_PAGE,
    VALIDATION(ValidationGuiManager),
    CHAT_PAGE,
    FRIENDS_PAGE,
}

/* #[allow(non_camel_case_types)]
pub(crate) enum BackgroundState {
    VOICE_VIDEO_CHAT,    
} */

pub(crate) struct ClientManager {
    server_coms: Rfc<ServerCommunication>,
    theme: Theme,
    foreground_state: ForegroundState,
    // background: BackgroundState,
}
impl ClientManager {
    // TODO: Create an initializer, that finds if the user has session.

    pub(crate) fn new(server_coms: Rfc<ServerCommunication>, theme: Theme) -> Self {
        Self {
            server_coms,
            theme, 
            foreground_state: ForegroundState::VALIDATION(ValidationGuiManager::new()),
        }
    } 

    pub(crate) fn init(&mut self) -> Result<(), StdError> {
        match &mut self.foreground_state {
            ForegroundState::VALIDATION(validation) => {
                let server_coms_login = Rfc::clone(&self.server_coms);
                let server_coms_sign_up = Rfc::clone(&self.server_coms);

                validation
                    .set_login_fn(move |info| {
                        info!("Login: {:?}", info);
                        server_coms_login.borrow_mut().send(ClientMessage::LOGIN(info))
                    })
                    .set_sign_up_fn(move |info| {
                        info!("Sign Up: {:?}", info);
                        server_coms_sign_up.borrow_mut().send(ClientMessage::SIGN_UP(info))
                    });
            }
            _ => (),
        };

        Ok(())
    }

    pub(crate) fn on_imgui(&mut self, ui: &mut imgui::Ui, renderer: &Renderer) {
        match &mut self.foreground_state {
            ForegroundState::MAIN_PAGE => todo!(),
            ForegroundState::VALIDATION(validation) => validation.on_imgui(ui, renderer, &self.theme),
            ForegroundState::CHAT_PAGE => todo!(),
            ForegroundState::FRIENDS_PAGE => todo!(),
        } 
    }
}