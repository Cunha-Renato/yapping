use std::rc::Rc;
use yapping_core::{client_server_coms::{Modification, Notification, Query, Response, ServerMessage, ServerMessageContent, Session}, l3gion_rust::{imgui, lg_core::renderer::Renderer, sllog::{error, info}, AsLgTime, Rfc, StdError, UUID}, user::User};
use crate::{gui::{find_user_gui::FindUserGuiManager, friends_notifications_gui::FriendsNotificationsManager, gui_manager::GuiMannager, show_loading_gui, sidebar_gui_manager::SidebarGuiManager, theme::Theme, validation_gui::validation_gui_manager::ValidationGuiManager}, server_coms::ServerCommunication};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ForegroundState {
    MAIN_PAGE,
    TOKEN,
    VALIDATION,
    CHAT_PAGE,
    FRIENDS_NOTIFICATIONS,
    FIND_USERS(String),
}

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) shared_mut: Rfc<SharedMut>,
    pub(crate) theme: Rc<Theme>,
}
pub(crate) struct SharedMut {
    pub(crate) user: Option<User>,
    pub(crate) foreground_state: ForegroundState,
}

struct GuiManagers {
    validation: ValidationGuiManager,
    sidebar: SidebarGuiManager,
    find_user: FindUserGuiManager,
}
impl GuiManagers {
    fn new(app_state: AppState) -> Self {
        Self {
            validation: ValidationGuiManager::new(app_state.clone()),
            sidebar: SidebarGuiManager::new(app_state.clone()),
            find_user: FindUserGuiManager::new(app_state.clone()),
        }
    }

    fn on_update(&mut self, server_coms: &mut ServerCommunication) -> Vec<Option<StdError>> {
        vec![
            self.validation.on_update(server_coms).err(),
            self.sidebar.on_update(server_coms).err(),
            self.find_user.on_update(server_coms).err(),
        ]
    }

    fn on_responded_messages(&mut self, messages: &mut Vec<(ServerMessage, Response)>, server_coms: &mut ServerCommunication) -> Vec<Option<StdError>> {
        vec![
            self.validation.on_responded_messages(messages, server_coms).err(),
            self.sidebar.on_responded_messages(messages, server_coms).err(),
            self.find_user.on_responded_messages(messages, server_coms).err(),
        ]
    }
    
    fn on_received_messages(&mut self, messages: &mut Vec<ServerMessage>, server_coms: &mut ServerCommunication) -> Vec<Option<StdError>> {
        vec![
            self.validation.on_received_messages(messages, server_coms).err(),
            self.sidebar.on_received_messages(messages, server_coms).err(),
            self.find_user.on_received_messages(messages, server_coms).err(),
        ]
    }
}

pub(crate) struct ClientManager {
    app_state: AppState,
    server_coms: Rfc<ServerCommunication>,
    gui_managers: GuiManagers,
    // background: BackgroundState,
}
impl ClientManager {
    // TODO: Create an initializer, that finds if the user has session.

    pub(crate) fn new(server_coms: Rfc<ServerCommunication>, theme: Theme) -> Self {
        let theme = Rc::new(theme);
        let app_state = AppState {
            shared_mut: Rfc::new(SharedMut {
                user: None,
                foreground_state: ForegroundState::VALIDATION,
            }),
            theme: Rc::clone(&theme),
        };

        Self {
            app_state: app_state.clone(),
            server_coms,
            gui_managers: GuiManagers::new(app_state),
        }
    } 

    pub(crate) fn init(&mut self) -> Result<(), StdError> {
        // TODO: Initialize user if the user has session.
        Ok(())
    }

    pub(crate) fn on_update(&mut self) {
        if !self.server_coms.borrow().connected() {
            // TODO: Query All of the information again
        }

        for e in self.gui_managers.on_update(&mut self.server_coms.borrow_mut()) {
            if let Some(e) = e { error!("{e}"); }
        }
    }

    pub(crate) fn on_responded_messages(&mut self, mut messages: Vec<(ServerMessage, Response)>) -> Result<(), StdError> {
        for (_, response) in &messages {
            info!("Received response: {:#?}", response);
        }
        
        for e in self.gui_managers.on_responded_messages(&mut messages, &mut self.server_coms.borrow_mut()) {
            if let Some(e) = e { error!("{e}"); }
        }

        Ok(())
    }

    pub(crate) fn on_received_messages(&mut self, messages: Vec<ServerMessage>) -> Result<(), StdError> {
        for msg in messages {
            // TODO: GuiManagers::on_received_messages.

            info!("Received message: {:#?}", msg);

            match msg.content {
                ServerMessageContent::NOTIFICATION(notification) => {
                    self.server_coms.borrow_mut().send(ServerMessage::new(msg.uuid, ServerMessageContent::RESPONSE(Response::OK)))?;
                }
                _ => (),
            }
        }

        Ok(())
    }

    pub(crate) fn on_imgui(&mut self, ui: &mut imgui::Ui, renderer: &Renderer) {
        if !self.server_coms.borrow().connected() {
            show_loading_gui(ui, renderer, [0.0, 0.0], ui.io().display_size, self.app_state.theme.main_bg_color);

            return;
        }

        match &self.app_state.shared_mut.borrow().foreground_state {
            ForegroundState::MAIN_PAGE => {
                self.gui_managers.sidebar.on_imgui(ui, renderer);
            },
            ForegroundState::VALIDATION => self.gui_managers.validation.on_imgui(ui, renderer),
            ForegroundState::CHAT_PAGE => {
                self.gui_managers.sidebar.on_imgui(ui, renderer);
            },
            ForegroundState::FRIENDS_NOTIFICATIONS => {
                self.gui_managers.sidebar.on_imgui(ui, renderer);
            },
            ForegroundState::FIND_USERS(_) => {
                self.gui_managers.sidebar.on_imgui(ui, renderer);
                self.gui_managers.find_user.on_imgui(ui, renderer);
            },
            _ => (),
        }
    }
    
    pub(crate) fn show_debug_gui(&self, ui: &imgui::Ui) {
        ui.tree_node_config("ClientManager")
            .framed(true)
            .build(|| {
                ui.text(std::format!("ForegroundState: {:?}", self.app_state.shared_mut.borrow().foreground_state));

                ui.tree_node_config("User")
                    .framed(true)
                    .build(|| if let Some(user) = &self.app_state.shared_mut.borrow().user {
                        ui.text(std::format!("{:?}", user.state()));
                        ui.text(std::format!("uuid: {}", user.uuid().to_string()));
                        ui.text(std::format!("tag: {}", user.tag()));
                        ui.text(std::format!("profile_pic: {:?}", user.profile_pic()));
                        ui.text("friends: ");
                        for friend in user.friends() {
                            ui.tree_node_config(friend.tag())
                                .framed(true)
                                .build(|| {
                                    ui.text(std::format!("{:?}", friend.state()));
                                    ui.text(std::format!("uuid: {}", friend.uuid().to_string()));
                                    ui.text(std::format!("profile_pic: {:?}", friend.profile_pic()));
                                });
                        }
                        ui.text(std::format!("chats: {:#?}", user.chats()));
                    });
                
                ui.tree_node_config("Theme")
                    .framed(true)
                    .build(|| ui.text(std::format!("{:#?}", &self.app_state.theme)));
            });
    }

    pub(crate) fn shutdown(&mut self) -> Result<(), StdError> {
        error!("Shutdown: Not Implemented!");
        
        Ok(())
    }
}