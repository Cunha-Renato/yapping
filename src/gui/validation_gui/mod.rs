use yapping_core::l3gion_rust::{imgui, lg_core::{renderer::Renderer, uuid::UUID}};
use crate::client_manager::{ClientManager, ForegroundState};

mod login;
mod sign_up;

#[derive(Default)]
pub(crate) struct Validation {
    error: String,
    user_tag: String,
    user_e_mail: String,
    password: UUID,
}
impl Validation {
    pub(crate) fn show_and_manage_validation_gui(&mut self, renderer: &Renderer, client_manager: &mut ClientManager, ui: &mut imgui::Ui) {
        let mut client_action = None;

        match client_manager.foreground_state {
            ForegroundState::LOGIN_PAGE => {
                let (user_tag, password, action) = login::show_login_gui(
                    renderer, 
                    &client_manager.theme, 
                    std::mem::take(&mut self.user_tag),
                    &self.error,
                    ui
                );
                client_action = action;

                self.user_tag = user_tag;
                self.password = password;
            },
            ForegroundState::SIGN_UP_PAGE => {
                let (user_tag, user_email, password, action) = sign_up::show_sign_up_gui(
                    renderer, 
                    &client_manager.theme, 
                    std::mem::take(&mut self.user_tag),
                    std::mem::take(&mut self.user_e_mail),
                    &self.error,
                    ui
                );
                client_action = action;

                self.user_tag = user_tag;
                self.user_e_mail = user_email;
                self.password = password;
            },
            _ => (),
        }
        
        if let Some(action) = client_action {
            match client_manager.user_action(action) {
                Ok(_) => self.error.clear(),
                Err(e) => self.error = e.to_string(),
            }

            self.password = UUID::from_u128(0);
            self.user_tag.clear();
            self.user_e_mail.clear();
        }
    }
}
