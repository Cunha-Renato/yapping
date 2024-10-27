use std::rc::Rc;

use yapping_core::{l3gion_rust::{imgui, lg_core::renderer::Renderer, sllog::info, AsLgTime, Rfc, StdError}, server_message::{ClientMessageContent, ServerMessage, ServerMessageContent, SuccessType}, user::{User, UserCreationInfo}};
use crate::{gui::{show_loading_gui, theme::Theme, validation_gui::validation_gui_manager::{ValidationAction, ValidationGuiManager}}, server_coms::ServerCommunication, ClientMessage};

struct GUIManagers {
    validation: ValidationGuiManager,
}
impl GUIManagers {
    fn new(theme: Rc<Theme>) -> Self {
        Self {
            validation: ValidationGuiManager::new(theme),
        }
    }
}

#[allow(non_camel_case_types)]
pub(crate) enum ForegroundState {
    MAIN_PAGE,
    VALIDATION,
    CHAT_PAGE,
    FRIENDS_PAGE,
}

pub(crate) struct ClientManager {
    server_coms: Rfc<ServerCommunication>,
    current_user: Option<User>,
    theme: Rc<Theme>,
    foreground_state: ForegroundState,
    gui_managers: GUIManagers,
    // background: BackgroundState,
}
impl ClientManager {
    // TODO: Create an initializer, that finds if the user has session.

    pub(crate) fn new(server_coms: Rfc<ServerCommunication>, theme: Theme) -> Self {
        let theme = Rc::new(theme);
        Self {
            server_coms,
            current_user: None,
            theme: Rc::clone(&theme), 
            foreground_state: ForegroundState::VALIDATION,
            gui_managers: GUIManagers::new(theme),
        }
    } 

    pub(crate) fn init(&mut self) -> Result<(), StdError> {
        // TODO: Initialize user if the user has session.
        Ok(())
    }

    pub(crate) fn on_imgui(&mut self, ui: &mut imgui::Ui, renderer: &Renderer) {
        if !self.server_coms.borrow().connected() {
            show_loading_gui(ui, renderer, [0.0, 0.0], ui.io().display_size, self.theme.main_bg_color);
            return;
        }

        match &mut self.foreground_state {
            ForegroundState::MAIN_PAGE => self.on_main_page(ui, renderer),
            ForegroundState::VALIDATION => self.on_validation(ui, renderer),
            ForegroundState::CHAT_PAGE => todo!(),
            ForegroundState::FRIENDS_PAGE => todo!(),
        } 
    }
}
impl ClientManager {
    fn on_validation(&mut self, ui: &mut imgui::Ui, renderer: &Renderer) {
        self.gui_managers.validation.on_imgui(ui, renderer, |v_action, info| {
            match match v_action {
                // Sending and waiting for response;
                ValidationAction::LOGIN => self.server_coms.borrow_mut().send_and_wait(1_u32.s(), &ClientMessage::new(ClientMessageContent::LOGIN(info))),
                ValidationAction::SIGN_UP => self.server_coms.borrow_mut().send_and_wait(1_u32.s(), &ClientMessage::new(ClientMessageContent::SIGN_UP(info)))
            }? {
                // Dealing with the received message;
                ServerMessageContent::SUCCESS(SuccessType::LOGIN(user) | SuccessType::SIGN_UP(user)) => {
                    self.foreground_state = ForegroundState::MAIN_PAGE;
                    self.current_user = Some(user);

                    Ok(())
                },
                ServerMessageContent::FAIL(e) => Err(e.into()),
                _ => Err("Got wrong response from the Server!".into()),
            }
        });
    }

    fn on_main_page(&mut self, ui: &mut imgui::Ui, renderer: &Renderer) {

    }
}