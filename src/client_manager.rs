use l3gion_rust::{lg_types::reference::Rfc, StdError};

use crate::{user_action::UserAction, gui::theme::Theme, server_coms::ServerCommunication};

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ForegroundState {
    MAIN_PAGE,
    LOGIN_PAGE,
    SIGN_UP_PAGE,
    CHAT_PAGE,
    FRIENDS_PAGE,
}

/* #[allow(non_camel_case_types)]
pub(crate) enum BackgroundState {
    VOICE_VIDEO_CHAT,    
} */

pub(crate) struct ClientManager {
    server_coms: Rfc<ServerCommunication>,
    pub(crate) theme: Theme,
    pub(crate) foreground_state: ForegroundState,
    // pub(crate) background: BackgroundState,
}
impl ClientManager {
    pub(crate) fn new(server_coms: Rfc<ServerCommunication>, theme: Theme, foreground_state: ForegroundState) -> Self {
        Self {
            server_coms,
            theme, 
            foreground_state,
        }
    }
    pub(crate) fn user_action(&mut self, user_action: UserAction) -> Result<(), StdError> {
        match user_action {
            UserAction::LOGIN => match self.foreground_state {
                ForegroundState::LOGIN_PAGE => {
                    // TODO: Request LOGIN
                    // Simulating the server
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    // TODO: self.foreground_state = ForegroundState::MAIN_PAGE
                    
                    Err("Failed to Login".into())
                },
                ForegroundState::SIGN_UP_PAGE => {
                    self.foreground_state = ForegroundState::LOGIN_PAGE;
                    
                    Ok(())
                }
                _ => Ok(()),
            },
            UserAction::SIGN_UP => match self.foreground_state {
                ForegroundState::LOGIN_PAGE => {
                    self.foreground_state = ForegroundState::SIGN_UP_PAGE;

                    Ok(())
                },
                ForegroundState::SIGN_UP_PAGE => {
                    // TODO: Request Sign Up
                    // Simulating the server
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    // TODO: self.foreground_state = ForegroundState::MAIN_PAGE

                    Err("Failed to Sign Up".into())
                },
                _ => Ok(()),
            },
        }
    }
}