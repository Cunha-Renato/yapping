use yapping_core::{l3gion_rust::{imgui, lg_core::renderer::Renderer}, user::UserCreationInfo};
use crate::client_manager::{ClientManager, ForegroundState};

mod gui_components;
mod login;
mod sign_up;

#[derive(Default)]
pub(crate) struct Validation {
    error: String,
    creation_info: (String, UserCreationInfo),
}
impl Validation {
    pub(crate) fn show_and_manage_validation_gui(&mut self, renderer: &Renderer, client_manager: &mut ClientManager, ui: &mut imgui::Ui) {
        let client_action = match client_manager.foreground_state {
            ForegroundState::LOGIN_PAGE => {
                let action = login::show_login_gui(
                    renderer, 
                    &client_manager.theme, 
                    &mut self.creation_info,
                    &self.error,
                    ui
                );

                action
            },
            ForegroundState::SIGN_UP_PAGE => {
                let action = sign_up::show_sign_up_gui(
                    renderer, 
                    &client_manager.theme, 
                    &mut self.creation_info,
                    &self.error,
                    ui
                );

                action
            },
            _ => None,
        };
        
        if let Some(action) = client_action {
            match client_manager.user_action(action) {
                Ok(_) => self.error.clear(),
                Err(e) => self.error = e.to_string(),
            }

            self.creation_info = (String::default(), UserCreationInfo::default());
        }
    }
}
