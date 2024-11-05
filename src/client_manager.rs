use std::rc::Rc;
use yapping_core::{client_server_coms::{Notification, Query, Response, ServerMessage, ServerMessageContent, Session}, l3gion_rust::{imgui, lg_core::renderer::Renderer, sllog::{error, info}, AsLgTime, Rfc, StdError, UUID}, user::{User, UserCreationInfo}};
use crate::{gui::{friends_page_gui::FriendsPageManager, show_loading_gui, sidebar_gui::{SidebarAction, SidebarManager}, theme::Theme, validation_gui::validation_gui_manager::{ValidationAction, ValidationGuiManager}}, server_coms::ServerCommunication};

struct GUIManagers {
    validation: ValidationGuiManager,
    sidebar: SidebarManager,
    friend_page: FriendsPageManager,
}
impl GUIManagers {
    fn new(theme: Rc<Theme>) -> Self {
        Self {
            validation: ValidationGuiManager::new(Rc::clone(&theme)),
            sidebar: SidebarManager::new(Rc::clone(&theme)),
            friend_page: FriendsPageManager::new(theme),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
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

    pub(crate) fn on_server_message(&mut self, messages: Vec<(ServerMessageContent, Response)>) -> Result<(), StdError> {
        // TODO:
        
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
            ForegroundState::FRIENDS_PAGE => self.on_friends_page(ui, renderer),
        } 
    }
    
    pub(crate) fn show_debug_gui(&self, ui: &imgui::Ui) {
        ui.tree_node_config("ClientManager")
            .framed(true)
            .build(|| {
                ui.text(std::format!("ForegroundState: {:?}", self.foreground_state));

                ui.tree_node_config("User")
                    .framed(true)
                    .build(|| if let Some(user) = &self.current_user {
                        ui.text(std::format!("{:?}", user.state()));
                        ui.text(std::format!("uuid: {}", user.uuid().to_string()));
                        ui.text(std::format!("tag: {}", user.tag()));
                        ui.text(std::format!("profile_pic: {:?}", user.profile_pic()));
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
                    .build(|| ui.text(std::format!("{:#?}", &self.theme)));
                
                ui.tree_node_config("GUIManagers")
                .framed(true)
                .build(|| {
                    ui.tree_node_config("Validation")
                        .framed(true)
                        .build(|| ui.text(std::format!("{:#?}", self.gui_managers.validation)));
                    
                    ui.tree_node_config("Sidebar")
                        .framed(true)
                        .build(|| ui.text(std::format!("{:#?}", self.gui_managers.sidebar)));
                    
                    ui.tree_node_config("FriendsPage")
                        .framed(true)
                        .build(|| ui.text(std::format!("{:#?}", self.gui_managers.friend_page)));
                });
            });
    }

    pub(crate) fn shutdown(&mut self) -> Result<(), StdError> {
        error!("Shutdown: Not Implemented!");
        
        Ok(())
    }
}
impl ClientManager {
    fn on_validation(&mut self, ui: &mut imgui::Ui, renderer: &Renderer) {
        self.gui_managers.validation.on_imgui(ui, renderer, |v_action, info| {
            match match v_action {
                // Sending and waiting for response;
                ValidationAction::LOGIN => self.server_coms.borrow_mut()
                    .send_and_wait(
                        1_u32.s(), 
                        ServerMessage::from(ServerMessageContent::SESSION(Session::LOGIN(info)))
                    ),
                ValidationAction::SIGN_UP => self.server_coms.borrow_mut()
                    .send_and_wait(
                        1_u32.s(), 
                        ServerMessage::from(ServerMessageContent::SESSION(Session::SIGN_UP(info))))
            }? {
                // Dealing with the received message;
                ServerMessageContent::RESPONSE(response) => match response {
                    Response::OK_SESSION(Session::TOKEN(user)) => {
                        self.foreground_state = ForegroundState::MAIN_PAGE;
                        self.current_user = Some(user);
                    
                        Ok(())
                    }
                    Response::Err(e) => Err(e.into()),
                    _ => Err("Got wrong response from the Server!".into()),
                }
                _ => Err("Got wrong response from the Server!".into()),
            }
        });
    }

    fn on_main_page(&mut self, ui: &mut imgui::Ui, renderer: &Renderer) {
        self.on_sidebar(ui, renderer);
    }

    fn on_sidebar(&mut self, ui: &mut imgui::Ui, renderer: &Renderer) {
        if let Some(user) = &self.current_user {
            self.gui_managers.sidebar.on_imgui(
                ui, 
                renderer, 
                user.friends(),
                |action| {
                    if let Err(e) = match action {
                        SidebarAction::FIND_NEW_FRIEND(friend_tag) => {
                            self.foreground_state = ForegroundState::FRIENDS_PAGE;
                            self.server_coms.borrow_mut().send(ServerMessage::from(ServerMessageContent::QUERY(Query::USERS_CONTAINS_TAG(friend_tag))))
                        },
                    } {
                        error!("During Sidebar::on_imgui! {}", e);
                    };
                });
        }
    }
    
    fn on_friends_page(&mut self, ui: &mut imgui::Ui, renderer: &Renderer) {
        self.on_sidebar(ui, renderer);

        self.gui_managers.friend_page.on_imgui(
            ui, 
            renderer
        );
    }
}