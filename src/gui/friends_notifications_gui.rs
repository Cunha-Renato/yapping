use std::collections::HashMap;

use yapping_core::{client_server_coms::{Notification, NotificationType, Query, Response, ServerMessage, ServerMessageContent}, l3gion_rust::{imgui, lg_core::renderer::Renderer, sllog::error, AsLgTime, StdError, UUID}, user::User};

use crate::{client_manager::AppState, server_coms::ServerCommunication};

use super::{gui_manager::GuiMannager, use_font, window, NEXT_WINDOW_SPECS};

pub(crate) struct FriendsNotificationsGuiManager {
    app_state: AppState,
    // User UUID, Notification UUID
    notifications: HashMap<UUID, UUID>,
    waiting_response: UUID,
    // Notification UUID, User
    user_requests: HashMap<UUID, User>,
    // Notification UUID
    notifications_accepted: Vec<UUID>,
    init: bool,
}
impl FriendsNotificationsGuiManager {
    pub(crate) fn new(app_state: AppState) -> Self {
        Self {
            app_state,
            notifications: HashMap::default(),
            user_requests: HashMap::default(),
            notifications_accepted: Vec::default(),
            waiting_response: UUID::default(),
            init: false,
        }
    }
}
impl GuiMannager for FriendsNotificationsGuiManager {
    fn on_imgui(&mut self, ui: &imgui::Ui, _renderer: &Renderer) {
        let (window_pos, window_size) = unsafe { NEXT_WINDOW_SPECS };

        window(
            ui, 
            "friends_notifications_window", 
            None,
            window_pos,
            window_size, 
            [10.0, 10.0],
            window_size, 
            self.app_state.theme.main_bg_color,
            |ui| {
                let _font = use_font(ui, super::FontType::REGULAR24);

                for (notification_uuid, user) in &self.user_requests {
                    if ui.button(user.tag()) {
                        self.notifications_accepted.push(*notification_uuid);
                    }
                }
                
            });
        
        unsafe { NEXT_WINDOW_SPECS = ([0.0; 2], [0.0; 2]); }
    }

    fn on_update(&mut self, server_coms: &mut ServerCommunication) -> Result<(), StdError> {
        self.try_init(server_coms);
        
        if !self.notifications.is_empty() {
            let users_uuid = self.notifications
                .iter()
                .map(|(u_uuid, _)| *u_uuid)
                .collect();

            match server_coms.send_and_wait(1.s(), ServerMessage::from(ServerMessageContent::QUERY(Query::USERS_BY_UUID(users_uuid))))? {
                ServerMessageContent::RESPONSE(Response::OK_QUERY(Query::RESULT_USER(users))) => {
                    for user in users {
                        if let Some(notification_uuid) = self.notifications.remove(&user.uuid()) {
                            self.user_requests.insert(notification_uuid, user);
                        }
                    }

                    self.notifications.clear();
                },
                _ => todo!()
            }
        }
        
        if !self.notifications_accepted.is_empty() {
            if let Some(current_user) = &mut self.app_state.shared_mut.borrow_mut().user {
                for notification_uuid in std::mem::take(&mut self.notifications_accepted) {
                    if let Some(user) = self.user_requests.remove(&notification_uuid) {
                        server_coms.send(ServerMessage::from(ServerMessageContent::NOTIFICATION(Notification::new_with_uuid(
                            notification_uuid,
                            NotificationType::FRIEND_ACCEPTED(current_user.uuid(), user.uuid())
                        ))))?;
                    }
                }
            }
        }

        Ok(())
    }
    
    fn on_responded_messages(&mut self, message: &(ServerMessage, Response), _server_coms: &mut ServerCommunication) -> Result<bool, StdError> {
        if message.0.uuid == self.waiting_response
        {
            self.waiting_response = UUID::default();
            match &message.1 {
                Response::OK_QUERY(Query::RESULT_FRIEND_REQUESTS(notifications)) => {
                    for notification in notifications {
                        match notification.notification_type {
                            NotificationType::FRIEND_REQUEST(sender, _) => { let _ = self.notifications.insert(sender, notification.uuid()); },
                            _ => (),
                        }
                    }
                    self.init = true;
                },
                Response::Err(e) => return Err(e.clone().into()),
                _ => return Err(String::from("In FriendsNotificationsGuiManager::on_responded_messages: Wrong response from Server!").into()),
            }
            
            Ok(true)
        }
        else { Ok(false) }
    }

    fn on_received_messages(&mut self, message: &ServerMessage) -> Result<(), StdError> {
        let user_uuid = if let Some(user) = &self.app_state.shared_mut.borrow().user { user.uuid() }
        else { return Ok(()); };

        match &message.content {
            ServerMessageContent::NOTIFICATION(notification) => {
                match notification {
                    Notification { notification_type: NotificationType::FRIEND_REQUEST(sender, receiver), .. } => {
                        if *receiver == user_uuid {
                            self.notifications.insert(*sender, notification.uuid());
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
        
        Ok(())
    }
}
impl FriendsNotificationsGuiManager {
    fn try_init(&mut self, server_coms: &mut ServerCommunication) {
        if !self.waiting_response.is_valid() && !self.init {
            let msg_uuid = UUID::generate();

            if let Err(e) = server_coms.send(ServerMessage::new(msg_uuid, ServerMessageContent::QUERY(Query::FRIEND_REQUESTS))) { error!("{e}"); }
            else { self.waiting_response = msg_uuid; }
        }
    }
}