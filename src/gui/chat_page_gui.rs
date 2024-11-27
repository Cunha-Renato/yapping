use yapping_core::{chat::Chat, chrono::{self, Datelike, Timelike}, client_server_coms::{Notification, NotificationType, ServerMessage, ServerMessageContent}, date_time::DateTime, l3gion_rust::{imgui, lg_core::renderer::Renderer, sllog::{error, warn}, StdError, UUID}, message::{Message, MessageType}, user::User};

use crate::{client_manager::{AppState, ForegroundState}, server_coms::ServerCommunication};

use super::{button, gui_manager::GuiMannager, multiline_text_input, no_resize_child_window, spacing, text_input, use_font, window, BORDER_RADIUS, NEXT_WINDOW_SPECS};

pub(crate) struct ChatGuiManager {
    app_state: AppState,
    chat_uuid: Option<UUID>,
    message_buffer: String,
    send_message: bool,
}
impl GuiMannager for ChatGuiManager {
    fn on_imgui(&mut self, ui: &imgui::Ui, renderer: &Renderer) {
        self.show_window(ui, renderer);
    }

    fn on_update(&mut self, server_coms: &mut ServerCommunication) -> Result<(), StdError> {
        match &mut self.app_state.shared_mut.borrow_mut().foreground_state {
            ForegroundState::CHAT_PAGE(chat_uuid) => if chat_uuid.is_valid() {
                server_coms.send(ServerMessage::from(ServerMessageContent::NOTIFICATION(Notification::new(NotificationType::MESSAGE_READ(*chat_uuid)))))?;
                self.chat_uuid = Some(std::mem::take(chat_uuid));
            },
            _ => ()
        };
       
        if let Some(user) = &self.app_state.shared_mut.borrow().user { if let Some(chat_uuid) = &self.chat_uuid { if self.send_message && !self.message_buffer.is_empty() {
            self.send_message = false;

            let message = Message::new(user.uuid(), MessageType::TEXT(std::mem::take(&mut self.message_buffer)), DateTime::from_utc(&chrono::Utc::now()));
            server_coms.send(ServerMessage::from(ServerMessageContent::NOTIFICATION(Notification::new(NotificationType::NEW_MESSAGE(*chat_uuid, message)))))?;
        }}}

        Ok(())
    }
}
impl ChatGuiManager {
    pub(crate) fn new(app_state: AppState) -> Self {
        Self {
            app_state,
            chat_uuid: None,
            message_buffer: String::default(),
            send_message: false,
        }
    }
}
impl ChatGuiManager {
    fn show_window(
        &mut self,
        ui: &imgui::Ui,
        renderer: &Renderer
    ) {
        let (window_pos, window_size) = unsafe { NEXT_WINDOW_SPECS };

        let shared = self.app_state.shared_mut.clone();
        let chats = &shared.borrow().chats;

        let chat = if let Some(chat_uuid) = self.chat_uuid {
            if let Some(chat) = chats.get(&chat_uuid) {
                chat
            }
            else { return; }
        }
        else { return; };

        window(
            ui, 
            "chat_page_window", 
            None,
            window_pos,
            window_size, 
            [10.0, 10.0],
            window_size, 
            self.app_state.theme.main_bg_color,
            |ui| {
                if let Some(user) = &shared.borrow().user {
                    self.show_chat_messages(ui, renderer, user, chat);
                }

                let should_focus_keyboard = ui.is_window_hovered() || ui.is_item_hovered();

                let _font = use_font(ui, super::FontType::REGULAR24);
                ui.set_cursor_pos([ui.cursor_pos()[0], ui.cursor_pos()[1] + 35.0]);
                self.send_message = multiline_text_input(
                    ui, 
                    [ui.content_region_avail()[0], 60.0],
                    &mut self.message_buffer, 
                    "##message_text_input", 
                    [1.0, 1.0, 1.0, 0.3], 
                    [1.0, 1.0, 1.0, 1.0], 
                    BORDER_RADIUS, 
                     imgui::InputTextFlags::ALWAYS_OVERWRITE
                    | imgui::InputTextFlags::CALLBACK_RESIZE
                    | imgui::InputTextFlags::CTRL_ENTER_FOR_NEW_LINE
                    | imgui::InputTextFlags::ENTER_RETURNS_TRUE
                );
                if should_focus_keyboard || ui.is_item_hovered() {
                    ui.set_keyboard_focus_here_with_offset(imgui::FocusedWidget::Previous);
                }
                ui.set_item_default_focus();
            });
    }
    
    fn show_chat_messages(
        &self, 
        ui: &imgui::Ui,
        renderer: &Renderer,
        current_user: &User,
        chat: &Chat,
    ) {
        no_resize_child_window(
            ui, 
            "chat_messages", 
            imgui::WindowFlags::empty(), 
            [ui.content_region_avail()[0], ui.content_region_avail()[1] - 100.0], 
            [0.0; 2], 
            self.app_state.theme.main_bg_color, 
            |ui| {
                for (i, message) in chat.messages().iter().enumerate() {
                    let mut _fonts = vec![use_font(ui, super::FontType::BOLD24)];
                    button(
                        ui, 
                        &std::format!("##user_pic_{i}"), 
                        [30.0, 30.0], 
                        BORDER_RADIUS, 
                        self.app_state.theme.positive_btn_color, 
                        self.app_state.theme.positive_btn_color, 
                        self.app_state.theme.positive_btn_color, 
                    );
                    
                    ui.same_line();
                    let cursor_pos_message = ui.cursor_pos()[0];
                    let cursor_pos_date_time = ui.cursor_pos()[1] + 9.0;
                    if let Some(sender) = current_user.friends()
                        .iter()
                        .find(|u| u.uuid() == message.sender())
                        .map(|u| u.tag())
                        .or_else(|| {
                            if current_user.uuid() == message.sender() { Some(current_user.tag()) }
                            else { Some("Unknown User") }
                        })
                    { ui.text(sender); }
                    
                    if let Ok(date_time) = message.date_time().to_local() {
                        _fonts.push(use_font(ui, super::FontType::BOLD15));
                        ui.same_line();
                        ui.set_cursor_pos([ui.cursor_pos()[0], cursor_pos_date_time]);

                        ui.text_colored([1.0, 1.0, 1.0, 0.5], std::format!("{}/{}/{} {}:{}", date_time.day(), date_time.month(), date_time.year(), date_time.hour(), date_time.minute()));
                    }
                    
                    _fonts.push(use_font(ui, super::FontType::REGULAR24));
                    ui.set_cursor_pos([cursor_pos_message, ui.cursor_pos()[1]]);
                    match message.content() {
                        MessageType::TEXT(text) => ui.text(text),
                        MessageType::FILE(_) => todo!(),
                    }
                    
                    spacing(ui, 5);
                }
            });
    }
}