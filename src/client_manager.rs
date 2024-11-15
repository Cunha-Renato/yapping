use std::rc::Rc;
use yapping_core::{client_server_coms::{Modification, Notification, NotificationType, Query, Response, ServerMessage, ServerMessageContent, Session}, l3gion_rust::{imgui, lg_core::renderer::Renderer, sllog::{error, info}, AsLgTime, Rfc, StdError, UUID}, serde::de::IntoDeserializer, user::User};
use crate::{gui::{find_user_gui::FindUserGuiManager, friends_notifications_gui::FriendsNotificationsGuiManager, gui_manager::GuiMannager, show_loading_gui, sidebar_gui_manager::SidebarGuiManager, theme::Theme, validation_gui::validation_gui_manager::ValidationGuiManager}, server_coms::{self, ServerCommunication}};

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
    friends_notifications: FriendsNotificationsGuiManager,
}
impl GuiManagers {
    fn new(app_state: AppState) -> Self {
        Self {
            validation: ValidationGuiManager::new(app_state.clone()),
            sidebar: SidebarGuiManager::new(app_state.clone()),
            find_user: FindUserGuiManager::new(app_state.clone()),
            friends_notifications: FriendsNotificationsGuiManager::new(app_state.clone()),
        }
    }

    fn on_responded_messages(&mut self, messages: &[(ServerMessage, Response)], server_coms: &mut ServerCommunication) -> Vec<StdError> {
        let mut errors = Vec::with_capacity(messages.len());

        for m in messages {
            if let Some(true) = self.validation.on_responded_messages(m, server_coms)
                .map_err(|err| errors.push(err))
                .ok() 
            { continue; }

            if let Some(true) = self.sidebar.on_responded_messages(m, server_coms)
                .map_err(|err| errors.push(err))
                .ok() 
            { continue; }

            if let Some(true) = self.find_user.on_responded_messages(m, server_coms)
                .map_err(|err| errors.push(err))
                .ok() 
            { continue; }

            if let Some(true) = self.friends_notifications.on_responded_messages(m, server_coms)
                .map_err(|err| errors.push(err))
                .ok() 
            { continue; }
        }
        
        errors
    }
    
    fn on_received_messages(&mut self, message: &ServerMessage) -> Vec<Option<StdError>> {
        vec![
            self.validation.on_received_messages(&message).err(),
            self.sidebar.on_received_messages(&message).err(),
            self.find_user.on_received_messages(&message).err(),
            self.friends_notifications.on_received_messages(&message).err(),
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

        let foreground = self.app_state.shared_mut.borrow().foreground_state.clone();
        if let Err(e) = match foreground {
            ForegroundState::MAIN_PAGE => Ok(()),
            ForegroundState::TOKEN => todo!(),
            ForegroundState::VALIDATION => self.gui_managers.validation.on_update(&mut self.server_coms.borrow_mut()),
            ForegroundState::CHAT_PAGE => todo!(),
            ForegroundState::FRIENDS_NOTIFICATIONS => self.gui_managers.friends_notifications.on_update(&mut self.server_coms.borrow_mut()),
            ForegroundState::FIND_USERS(_) => self.gui_managers.find_user.on_update(&mut self.server_coms.borrow_mut()),
        } {
            error!("{e}");
        }
        
        if let Err(e) = self.gui_managers.sidebar.on_update(&mut self.server_coms.borrow_mut()) {
            error!("{e}");
        }
    }

    pub(crate) fn on_responded_messages(&mut self, messages: Vec<(ServerMessage, Response)>) -> Result<(), StdError> {
        for (_, response) in &messages {
            info!("Received response: {:#?}", response);
        }
        
        for e in self.gui_managers.on_responded_messages(&messages, &mut self.server_coms.borrow_mut()) {
            error!("{e}");
        }

        Ok(())
    }

    pub(crate) fn on_received_messages(&mut self, messages: Vec<ServerMessage>) -> Result<(), StdError> {
        let mut server_coms = self.server_coms.borrow_mut();

        for msg in messages {
            info!("Received message: {:#?}", msg);
            self.gui_managers.on_received_messages(&msg);

            match msg.content {
                ServerMessageContent::SESSION(Session::TOKEN(user)) => {
                    self.app_state.shared_mut.borrow_mut().user = Some(user.clone());
                }
                _ => (),
            };

            server_coms.send(ServerMessage::new(msg.uuid, ServerMessageContent::RESPONSE(Response::OK)))?;
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
                self.gui_managers.friends_notifications.on_imgui(ui, renderer);
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