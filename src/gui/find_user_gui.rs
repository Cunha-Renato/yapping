use std::{borrow::Borrow, rc::Rc};
use yapping_core::{client_server_coms::{Notification, Query, Response, ServerMessage, ServerMessageContent}, l3gion_rust::{imgui, lg_core::renderer::Renderer, Rfc, StdError, UUID}, user::User};
use crate::{client_manager::AppState, server_coms::ServerCommunication};
use super::{button, gui_manager::GuiMannager, no_resize_child_window, window, BORDER_RADIUS, NEXT_WINDOW_SPECS};

pub(crate) struct FindUserGuiManager {
    app_state: AppState,
    users: Vec<User>,
    user_tag_to_search: Option<String>,
    user_selected: Option<UUID>,
    waiting_response: UUID,
}
impl FindUserGuiManager {
    pub(crate) fn new(app_state: AppState) -> Self {
        Self {
            app_state,
            users: Vec::default(),
            user_tag_to_search: None,
            user_selected: None,
            waiting_response: UUID::default(),
        }
    }
    
    pub(crate) fn set_user_tag(&mut self, user_tag: String) {
        if !user_tag.is_empty() { 
            self.user_tag_to_search = Some(user_tag);
        }
    }
}

impl GuiMannager for FindUserGuiManager {
    fn on_imgui(&mut self, ui: &imgui::Ui, renderer: &Renderer) {
        let (window_pos, window_size) = unsafe { NEXT_WINDOW_SPECS };

        window(
            ui, 
            "test", 
            None,
            window_pos,
            window_size, 
            [10.0, 10.0],
            window_size, 
            [0.3, 0.3, 0.3, 1.0],
            |ui| {
                for friend in &self.users {
                    let _window_rounding = ui.push_style_var(imgui::StyleVar::ChildRounding(3.0));
                    if self.friend_cell(friend, ui) {
                        self.user_selected = Some(friend.uuid());
                    }
                }
            });
        
        unsafe { NEXT_WINDOW_SPECS = ([0.0; 2], [0.0; 2]); }
    }

    fn on_update(&mut self, server_coms: &mut ServerCommunication) -> Result<(), StdError> {
        // Finding the matching user tags.
        if let Some(user_tag_to_search) = self.user_tag_to_search.take() {
            if !user_tag_to_search.is_empty() {
                let msg_uuid = UUID::generate();
                self.waiting_response = msg_uuid;

                server_coms.send(ServerMessage::new(msg_uuid, ServerMessageContent::QUERY(Query::USERS_CONTAINS_TAG(user_tag_to_search))))?;
            }
        }

        // If User wants to send a friend request.
        if let Some(user_uuid) = self.user_selected.take() {
            if let Some(current_user) = &self.app_state.shared_mut.borrow().user {
                server_coms.send(ServerMessage::new(UUID::generate(), ServerMessageContent::NOTIFICATION(Notification::FRIEND_REQUEST(current_user.uuid(), user_uuid))))?;
            }
        }
        
        Ok(())
    }
    
    fn on_responded_messages(&mut self, messages: &mut Vec<(ServerMessage, Response)>, _server_coms: &mut ServerCommunication) -> Result<(), StdError> {
        if let Some(i) = messages.iter_mut()
            .position(|(m, _)| m.uuid == self.waiting_response) 
        {
            let (_, response) = messages.remove(i);
            
            match response {
                Response::OK_QUERY(Query::RESULT(users)) => {
                    self.users = users.into_iter()
                        .filter(|u| {
                            if let Some(user) = &self.app_state.shared_mut.borrow().user {
                                return user.uuid() != u.uuid();
                            }
                            
                            true
                        })
                        .collect();
                },
                Response::Err(e) => return Err(e.into()),
                _ => return Err(String::from("In FindUserGuiManager::on_responded_messages: Wrong response from Server!").into()),
            }
        }
        
        Ok(())
    }
}
impl FindUserGuiManager {
    fn friend_cell(&self, friend: &User, ui: &imgui::Ui) -> bool {
        let theme = Rc::clone(&self.app_state.theme);

        no_resize_child_window(
            ui, 
            &std::format!("##child_window_{}", friend.uuid().to_string()),
            None, 
            [ui.content_region_avail()[0], 50.0], 
            [0.0; 2], 
            theme.accent_color, 
            |ui| {
                // Table setup.
                let _padding = ui.push_style_var(imgui::StyleVar::CellPadding([0.0, 0.0]));
                let _table = ui.begin_table_with_flags(
                    std::format!("##pic_table_friends_page_{}", friend.uuid().to_string()), 
                    2, 
                    imgui::TableFlags::empty()
                );
                ui.table_next_row();
                ui.table_next_column();

                button(
                    ui, 
                    &std::format!("##profile_pic_{}", friend.uuid().to_string()), 
                    ui.content_region_avail(),
                    0.0, 
                    theme.positive_actv_btn_color, 
                    theme.positive_actv_btn_color, 
                    theme.positive_actv_btn_color, 
                );

                ui.table_next_column();

                ui.text(friend.tag());
                ui.text(std::format!("{:?}", friend.state()));
                
                if ui.is_window_hovered() && ui.is_mouse_clicked(imgui::MouseButton::Right) {
                    ui.open_popup("##add_user_popup");
                }
                
                if let Some(_popup) = ui.begin_popup("##add_user_popup") {
                    // TODO: 
                    if button(
                        ui, 
                        "Add Friend", 
                        [30.0, 15.0], 
                        BORDER_RADIUS, 
                        theme.accent_color, 
                        theme.main_bg_color, 
                        theme.main_bg_color, 
                    ) {
                        return true;
                    }
                }
                
                false
            })
            .unwrap_or(false)
    }
}