use std::collections::HashMap;

use yapping_core::{chat::Chat, client_server_coms::{Notification, NotificationType, ServerMessage, ServerMessageContent}, l3gion_rust::{imgui, lg_core::renderer::Renderer, sllog::warn, Rfc, StdError, UUID}, user::User};
use crate::{client_manager::{AppState, ForegroundState}, server_coms::{self, ServerCommunication}};
use super::{button, centered_component, gui_manager::GuiMannager, no_resize_child_window, no_resize_window, spacing, text_input, use_font, BORDER_RADIUS, NEXT_WINDOW_SPECS};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub(crate) enum SidebarAction {
    FIND_NEW_FRIEND(String),
    CHAT_PAGE(UUID),
    NOTIFICATIONS,
    CONFIG,
}

#[derive(Debug, Clone)]
enum SidebarState {
    FRIENDS,
    CHATS
}

pub(crate) struct SidebarGuiManager {
    app_state: AppState,
    sidebar_state: SidebarState,
    sidebar_action: Option<SidebarAction>,
    search_buffer: String,
    begin_chat: Option<UUID>,
}
impl SidebarGuiManager {
    pub(crate) fn new(app_state: AppState) -> Self {
        Self {
            app_state,
            sidebar_state: SidebarState::FRIENDS,
            sidebar_action: None,
            search_buffer: String::default(),
            begin_chat: None,
        }
    }
}

impl GuiMannager for SidebarGuiManager {
    fn on_imgui(&mut self, ui: &imgui::Ui, renderer: &Renderer) {
        let window_size = [200.0, ui.io().display_size[1]];

        let app_state_clone = Rfc::clone(&self.app_state.shared_mut);
        let app_state_borrow = app_state_clone.borrow();

        no_resize_window(
            ui, 
            "Sidebar Window", 
            Some(imgui::WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS),
            [0.0, 0.0], 
            window_size, 
            [window_size[0] * 0.05, 5.0],
            window_size, 
            self.app_state.theme.left_panel_bg_color, 
            |ui| {
                let user = if let Some(user) = &app_state_borrow.user { user }
                else { return; };
                let chats = &app_state_borrow.chats;

                if let Some(action) = match self.sidebar_state {
                    SidebarState::FRIENDS => self.show_friends_sidebar(ui, renderer, user),
                    SidebarState::CHATS => self.show_chats_sidebar(ui, renderer, user, &chats),
                } {
                    self.sidebar_action = Some(action);
                }
            });
        
        unsafe { NEXT_WINDOW_SPECS = ([window_size[0], 0.0], [ui.io().display_size[0] - window_size[0], ui.io().display_size[1]]) };
    }

    fn on_update(&mut self, server_coms: &mut ServerCommunication) -> Result<(), StdError> {
        if let Some(action) = self.sidebar_action.take() {
            self.app_state.shared_mut.borrow_mut().foreground_state = match action {
                SidebarAction::FIND_NEW_FRIEND(mut user_tag) => ForegroundState::FIND_USERS(std::mem::take(&mut user_tag)),
                SidebarAction::NOTIFICATIONS => ForegroundState::FRIENDS_NOTIFICATIONS,
                SidebarAction::CHAT_PAGE(chat_uuid) => ForegroundState::CHAT_PAGE(chat_uuid),
                SidebarAction::CONFIG => todo!(),
            }
        }
        
        if let (Some(chat_with), Some(current_user)) = (std::mem::take(&mut self.begin_chat), &self.app_state.shared_mut.borrow().user) {
            self.begin_chat = None;
            server_coms.send(ServerMessage::from(ServerMessageContent::NOTIFICATION(Notification::new(NotificationType::NEW_CHAT(Chat::new(
                "placeholder_chat_tag", 
                vec![current_user.uuid(), chat_with]
            ))))))?;
        }
        
        Ok(())
    }
}
impl SidebarGuiManager {
    fn show_friends_sidebar(
        &mut self, 
        ui: &imgui::Ui, 
        renderer: &Renderer, 
        user: &User
    ) -> Option<SidebarAction>
    {
        let mut _fonts = vec![use_font(ui, super::FontType::BOLD24)];

        let mut size = ui.content_region_avail();
        size[1] = 50.0;
        
        if self.show_big_btn("Chats", ui) {
           self.sidebar_state = SidebarState::CHATS;
        }
        
        // Friends Search
        spacing(ui, 5);
        _fonts.push(use_font(ui, super::FontType::REGULAR24));
        ui.text("Friends");

        spacing(ui, 1);
        _fonts.push(use_font(ui, super::FontType::REGULAR17));
        if self.show_search("User Tag", ui) {
            return Some(SidebarAction::FIND_NEW_FRIEND(std::mem::take(&mut self.search_buffer)));
        }
        
        self.show_friend_list(ui, user.friends());
        
        spacing(ui, 1);
        _fonts.push(use_font(ui, super::FontType::BOLD17));
        if self.show_friend_requests_btn(ui) {
            return Some(SidebarAction::NOTIFICATIONS);
        }

        // User
        spacing(ui, 5);
        self.show_user(user, ui)
    }

    fn show_chats_sidebar(
        &mut self, 
        ui: &imgui::Ui,
        renderer: &Renderer, 
        user: &User,
        chats: &HashMap<UUID, Chat>,
    ) -> Option<SidebarAction>
    {
        let mut _fonts = vec![use_font(ui, super::FontType::BOLD24)];

        let mut size = ui.content_region_avail();
        size[1] = 50.0;
        
        if self.show_big_btn("Friends", ui) {
           self.sidebar_state = SidebarState::FRIENDS;
        }
        
        // Friends Search
        spacing(ui, 5);
        _fonts.push(use_font(ui, super::FontType::REGULAR24));
        ui.text("Chats");

        spacing(ui, 1);
        _fonts.push(use_font(ui, super::FontType::REGULAR17));
        self.show_search("Chat Tag", ui);
        
        // TODO: Get the chats
        self.show_chat_list(ui, chats);
        
        // User
        ui.set_cursor_pos([ui.cursor_pos()[0], ui.cursor_pos()[1] + 40.0]);
        spacing(ui, 7);
        self.show_user(user, ui)
    }

    fn show_user(&self, user: &User, ui: &imgui::Ui) -> Option<SidebarAction> {
        let _font = use_font(ui, super::FontType::REGULAR17);
        let _padding = ui.push_style_var(imgui::StyleVar::CellPadding([5.0, 0.0]));
        let _table = ui.begin_table("##user_sidebar", 3);
        let region_avail = ui.content_region_avail()[0];

        ui.table_setup_column_with(imgui::TableColumnSetup { 
            name: "##user_pic_column1", 
            flags: imgui::TableColumnFlags::WIDTH_FIXED, 
            init_width_or_weight: region_avail / 3.0 - 10.0, 
            ..Default::default()
        });
        ui.table_setup_column_with(imgui::TableColumnSetup { 
            name: "##user_pic_column2", 
            flags: imgui::TableColumnFlags::WIDTH_FIXED, 
            init_width_or_weight: region_avail / 2.0 - 10.0,
            ..Default::default()
        });
        ui.table_setup_column("##sidebar_config_btn");
          

        ui.table_next_row();
        ui.table_next_column();
        button(
            ui, 
            "##user_pic_sidebar", 
            ui.content_region_avail(), 
            BORDER_RADIUS, 
            self.app_state.theme.positive_btn_color, 
            self.app_state.theme.positive_actv_btn_color, 
            self.app_state.theme.positive_actv_btn_color, 
        );

        ui.table_next_column();
        ui.text(user.tag());
        ui.text(std::format!("{:?}", user.state()));

        ui.table_next_column();
        if button(
            ui, 
            "##config_sidebar", 
            ui.content_region_avail(), 
            BORDER_RADIUS, 
            self.app_state.theme.positive_btn_color, 
            self.app_state.theme.positive_actv_btn_color, 
            self.app_state.theme.positive_actv_btn_color, 
        ) {
            return Some(SidebarAction::CONFIG);
        }
        
        None
    }

    fn show_friend_requests_btn(&self, ui: &imgui::Ui) -> bool {
        button(
            ui, 
            "Friend Requests", 
            [ui.content_region_avail()[0], 40.0], 
            BORDER_RADIUS, 
            self.app_state.theme.accent_color, 
            self.app_state.theme.main_bg_color,
            self.app_state.theme.main_bg_color,
        )
    }

    fn show_big_btn(&self, label: &str, ui: &imgui::Ui) -> bool { 
        let mut size = ui.content_region_avail();
        size[1] = 50.0;

        centered_component(ui, size, |ui, c_size| button(
            ui, 
            label, 
            c_size, 
            50.0, 
            self.app_state.theme.accent_color, 
            self.app_state.theme.main_bg_color, 
            self.app_state.theme.accent_color, 
        ))
    }

    fn show_search(&mut self, label: &str, ui: &imgui::Ui) -> bool {
        let _window_rounding = ui.push_style_var(imgui::StyleVar::ChildRounding(BORDER_RADIUS));
        no_resize_child_window(
            ui, 
            "##search_child_window", 
            None,
            [ui.content_region_avail()[0], 33.0], 
            [2.0; 2], 
            self.app_state.theme.accent_color, 
            |ui| {
                if button(
                    ui, 
                    "+", 
                    [30.0, ui.content_region_avail()[1]], 
                    3.0, 
                    self.app_state.theme.accent_color, 
                    self.app_state.theme.main_bg_color, 
                    self.app_state.theme.accent_color, 
                ) {
                    return true;
                }
                
                ui.same_line();
                ui.set_cursor_pos([ui.cursor_pos()[0], ui.cursor_pos()[1] + 1.0]);
                ui.set_next_item_width(ui.content_region_avail()[0] - 1.0);
                let _padding = ui.push_style_var(imgui::StyleVar::FramePadding([5.0, 5.0]));

                text_input(
                    ui, 
                    label,
                    &mut self.search_buffer, 
                    "##search", 
                    self.app_state.theme.input_text_bg_light, 
                    [0.0, 0.0, 0.0, 1.0],
                    BORDER_RADIUS, 
                    imgui::InputTextFlags::CALLBACK_RESIZE
                    | imgui::InputTextFlags::ENTER_RETURNS_TRUE
                )
            })
            .unwrap_or(false)
    }

    fn show_friend_list(&mut self, ui: &imgui::Ui, friends: &[User]) {
        spacing(ui, 5);
        ui.separator();
        spacing(ui, 5);

        no_resize_child_window(
            ui, 
            "friend_list", 
            None, 
            [ui.content_region_avail()[0], ui.content_region_avail()[1] - 120.0], 
            [0.0, 0.0], 
            self.app_state.theme.left_panel_bg_color, 
            |ui| {
                let _window_rounding = ui.push_style_var(imgui::StyleVar::ChildRounding(BORDER_RADIUS));
                for (i, friend) in friends
                    .iter()
                    .enumerate() 
                    .filter(|(_, friend)| friend.tag().to_lowercase().contains(&self.search_buffer.to_lowercase()))
                {
                    no_resize_child_window(
                        ui, 
                        &std::format!("friend_{}", i), 
                        None, 
                        [ui.content_region_avail()[0], 50.0], 
                        [0.0, 0.0], 
                        self.app_state.theme.accent_color, 
                        |ui| {
                            if ui.is_window_hovered() && ui.is_mouse_clicked(imgui::MouseButton::Right) {
                                ui.open_popup("##sidebar_friend_popup");
                            }
                            
                            if let Some(_popup) = ui.begin_popup("##sidebar_friend_popup") {
                                if button(
                                    ui, 
                                    "Chat", 
                                    [30.0, 15.0], 
                                    BORDER_RADIUS, 
                                    self.app_state.theme.accent_color, 
                                    self.app_state.theme.main_bg_color, 
                                    self.app_state.theme.main_bg_color, 
                                ) {
                                    self.begin_chat = Some(friend.uuid());
                                }
                            }

                            button(
                                ui, 
                                &std::format!("##friend_pic_{}", i), 
                                [50.0, ui.content_region_avail()[1]], 
                                0.0,
                                self.app_state.theme.positive_btn_color, 
                                self.app_state.theme.positive_btn_color, 
                                self.app_state.theme.positive_btn_color, 
                            );

                            // TODO: Use tables here!
                            ui.same_line();
                            ui.text(friend.tag());
                            ui.same_line();
                            ui.text(std::format!("{:?}", friend.state()));        
                        });
                }
            });
    }

    fn show_chat_list(&mut self, ui: &imgui::Ui, chats: &HashMap<UUID, Chat>) {
        spacing(ui, 5);
        ui.separator();
        spacing(ui, 5);

        no_resize_child_window(
            ui, 
            "chats_list", 
            None, 
            [ui.content_region_avail()[0], ui.content_region_avail()[1] - 120.0], 
            [0.0, 0.0], 
            self.app_state.theme.left_panel_bg_color, 
            |ui| {
                let _window_rounding = ui.push_style_var(imgui::StyleVar::ChildRounding(BORDER_RADIUS));
                for (i, (_, chat)) in chats
                    .iter()
                    .enumerate() 
                    .filter(|(_, (_, chat))| (chat).tag().to_lowercase().contains(&self.search_buffer.to_lowercase()))
                {
                    no_resize_child_window(
                        ui, 
                        &std::format!("friend_{}", i), 
                        None, 
                        [ui.content_region_avail()[0], 50.0], 
                        [0.0, 0.0], 
                        self.app_state.theme.accent_color, 
                        |ui| {
                            if ui.is_window_hovered() && ui.is_mouse_clicked(imgui::MouseButton::Right) {
                                ui.open_popup("##sidebar_chat_popup");
                            }
                            
                            if let Some(_popup) = ui.begin_popup("##sidebar_chat_popup") {
                                // TODO: 
                            }

                            if button(
                                ui, 
                                &std::format!("##chat_pic{}", i), 
                                [50.0, ui.content_region_avail()[1]], 
                                0.0,
                                self.app_state.theme.positive_btn_color, 
                                self.app_state.theme.positive_btn_color, 
                                self.app_state.theme.positive_btn_color, 
                            ) {
                                self.sidebar_action = Some(SidebarAction::CHAT_PAGE(chat.uuid()));
                            };

                            // TODO: Use tables here!
                            ui.same_line();
                            ui.text(chat.tag());
                            
                            if ui.is_window_hovered() && ui.is_mouse_clicked(imgui::MouseButton::Left) {
                                self.sidebar_action = Some(SidebarAction::CHAT_PAGE(chat.uuid()));
                            }
                        });
                }
            });
    }
}