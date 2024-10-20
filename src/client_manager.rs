use yapping_core::l3gion_rust::{imgui, lg_core::renderer::Renderer, sllog::info, Rfc, StdError};
use crate::{gui::{theme::Theme, validation_gui::validation_gui_manager::ValidationGuiManager}, server_coms::ServerCommunication, ClientMessage};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) enum ForegroundState {
    MAIN_PAGE,
    LOGIN_PAGE(ValidationGuiManager),
    SIGN_UP_PAGE(ValidationGuiManager),
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
            foreground_state: ForegroundState::LOGIN_PAGE(ValidationGuiManager::default()),
        }
    }

    pub(crate) fn on_update(&mut self) -> Result<(), StdError> {
        match &mut self.foreground_state {
            ForegroundState::MAIN_PAGE => todo!(),
            ForegroundState::LOGIN_PAGE(login) => if login.is_done() {
                info!("{:?}", login.get_creation_info())
            },
            ForegroundState::SIGN_UP_PAGE(sign_up) => if sign_up.is_done() {
                info!("{:?}", sign_up.get_creation_info())
            },
            ForegroundState::CHAT_PAGE => todo!(),
            ForegroundState::FRIENDS_PAGE => todo!(),
        }

        return Ok(())
    }
    
    pub(crate) fn on_imgui(&mut self, ui: &mut imgui::Ui, renderer: &Renderer) {
        match &mut self.foreground_state {
            ForegroundState::MAIN_PAGE => todo!(),
            ForegroundState::LOGIN_PAGE(login) => login.on_imgui(ui, renderer, &self.theme),
            ForegroundState::SIGN_UP_PAGE(sign_up) => sign_up.on_imgui(ui, renderer, &self.theme),
            ForegroundState::CHAT_PAGE => todo!(),
            ForegroundState::FRIENDS_PAGE => todo!(),
        } 
    }
}