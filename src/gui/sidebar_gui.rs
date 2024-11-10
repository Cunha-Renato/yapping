use std::{fmt::Debug, rc::Rc};

use yapping_core::{chat::Chat, l3gion_rust::{imgui, lg_core::renderer::Renderer, sllog::warn}, user::User};
use super::{button, centered_component, no_resize_child_window, no_resize_window, spacing, text_input, theme, use_font, BORDER_RADIUS, NEXT_WINDOW_SPECS};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub(crate) enum SidebarAction {
    FIND_NEW_FRIEND(String),
    NOTIFICATIONS,
    CONFIG,
}

#[derive(Debug, Clone)]
enum SidebarState {
    FRIENDS,
    CHATS
}

pub(crate) struct SidebarManager {
    state: SidebarState,
    theme: Rc<theme::Theme>,
    search_buffer: String,
}
impl SidebarManager {
    pub(crate) fn new(theme: Rc<theme::Theme>) -> Self {
        Self { 
            state: SidebarState::FRIENDS,
            theme, 
            search_buffer: String::default(),
        }
    }

    pub(crate) fn on_imgui(
        &mut self, 
        ui: &imgui::Ui, 
        renderer: &Renderer, 
        user: &User,
        mut func: impl FnMut(SidebarAction)
    ) {
        let window_size = [200.0, ui.io().display_size[1]];

        no_resize_window(
            ui, 
            "Sidebar Window", 
            Some(imgui::WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS),
            [0.0, 0.0], 
            window_size, 
            [window_size[0] * 0.05, 5.0],
            window_size, 
            self.theme.left_panel_bg_color, 
            |ui| {        
                if let Some(action) = match self.state {
                    SidebarState::FRIENDS => self.show_friends_sidebar(renderer, user, ui),
                    SidebarState::CHATS => self.show_chats_sidebar(renderer, user, ui),
                } {
                    func(action);
                }
            });
        
        unsafe { NEXT_WINDOW_SPECS = ([window_size[0], 0.0], [ui.io().display_size[0] - window_size[0], ui.io().display_size[1]]) };
    }
}
impl SidebarManager {
    fn show_friends_sidebar(
        &mut self, 
        renderer: &Renderer, 
        user: &User,
        ui: &imgui::Ui
    ) -> Option<SidebarAction>
    {
        let mut _fonts = vec![use_font(ui, super::FontType::BOLD24)];

        let mut size = ui.content_region_avail();
        size[1] = 50.0;
        
        if self.show_big_btn("Chats", ui) {
           self.state = SidebarState::CHATS;
        }
        
        // Friends Search
        spacing(ui, 5);
        _fonts.push(use_font(ui, super::FontType::REGULAR24));
        ui.text("Friends");

        spacing(ui, 1);
        _fonts.push(use_font(ui, super::FontType::REGULAR17));
        if self.show_search("User Tag", ui) {
            println!("{}", self.search_buffer);
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
        renderer: &Renderer, 
        user: &User,
        ui: &imgui::Ui
    ) -> Option<SidebarAction>
    {
        let mut _fonts = vec![use_font(ui, super::FontType::BOLD24)];

        let mut size = ui.content_region_avail();
        size[1] = 50.0;
        
        if self.show_big_btn("Friends", ui) {
           self.state = SidebarState::FRIENDS;
        }
        
        // Friends Search
        spacing(ui, 5);
        _fonts.push(use_font(ui, super::FontType::REGULAR24));
        ui.text("Chats");

        spacing(ui, 1);
        _fonts.push(use_font(ui, super::FontType::REGULAR17));
        self.show_search("Chat Tag", ui);
        
        // TODO: Get the chats
        self.show_chat_list(ui, &[]);
        
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
            self.theme.positive_btn_color, 
            self.theme.positive_actv_btn_color, 
            self.theme.positive_actv_btn_color, 
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
            self.theme.positive_btn_color, 
            self.theme.positive_actv_btn_color, 
            self.theme.positive_actv_btn_color, 
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
            self.theme.accent_color, 
            self.theme.main_bg_color,
            self.theme.main_bg_color,
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
            self.theme.accent_color, 
            self.theme.main_bg_color, 
            self.theme.accent_color, 
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
            self.theme.accent_color, 
            |ui| {
                if button(
                    ui, 
                    "+", 
                    [30.0, ui.content_region_avail()[1]], 
                    3.0, 
                    self.theme.accent_color, 
                    self.theme.main_bg_color, 
                    self.theme.accent_color, 
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
                    self.theme.input_text_bg_light, 
                    [0.0, 0.0, 0.0, 1.0],
                    BORDER_RADIUS, 
                    imgui::InputTextFlags::CALLBACK_RESIZE
                    | imgui::InputTextFlags::ENTER_RETURNS_TRUE
                )
            })
            .unwrap_or(false)
    }

    fn show_friend_list(&self, ui: &imgui::Ui, friends: &[User]) {
        spacing(ui, 5);
        ui.separator();
        spacing(ui, 5);

        no_resize_child_window(
            ui, 
            "friend_list", 
            None, 
            [ui.content_region_avail()[0], ui.content_region_avail()[1] - 120.0], 
            [0.0, 0.0], 
            self.theme.left_panel_bg_color, 
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
                        self.theme.accent_color, 
                        |ui| {
                            if ui.is_window_hovered() && ui.is_mouse_clicked(imgui::MouseButton::Right) {
                                ui.open_popup("##sidebar_friend_popup");
                            }
                            
                            if let Some(_popup) = ui.begin_popup("##sidebar_friend_popup") {
                                // TODO: 
                                if button(
                                    ui, 
                                    "Chat", 
                                    [30.0, 15.0], 
                                    BORDER_RADIUS, 
                                    self.theme.accent_color, 
                                    self.theme.main_bg_color, 
                                    self.theme.main_bg_color, 
                                ) {
                                    warn!("Begin Chat with: {}", friend.tag());
                                }
                            }

                            button(
                                ui, 
                                &std::format!("##friend_pic_{}", i), 
                                [50.0, ui.content_region_avail()[1]], 
                                0.0,
                                self.theme.positive_btn_color, 
                                self.theme.positive_btn_color, 
                                self.theme.positive_btn_color, 
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
    
    fn show_chat_list(&self, ui: &imgui::Ui, chats: &[Chat]) {
        spacing(ui, 5);
        ui.separator();
        spacing(ui, 5);

        no_resize_child_window(
            ui, 
            "chats_list", 
            None, 
            [ui.content_region_avail()[0], ui.content_region_avail()[1] - 120.0], 
            [0.0, 0.0], 
            self.theme.left_panel_bg_color, 
            |ui| {
                let _window_rounding = ui.push_style_var(imgui::StyleVar::ChildRounding(BORDER_RADIUS));
                for (i, friend) in chats
                    .iter()
                    .enumerate() 
                    .filter(|(_, chat)| (chat).tag().to_lowercase().contains(&self.search_buffer.to_lowercase()))
                {
                    no_resize_child_window(
                        ui, 
                        &std::format!("friend_{}", i), 
                        None, 
                        [ui.content_region_avail()[0], 50.0], 
                        [0.0, 0.0], 
                        self.theme.accent_color, 
                        |ui| {
                            if ui.is_window_hovered() && ui.is_mouse_clicked(imgui::MouseButton::Right) {
                                ui.open_popup("##sidebar_chat_popup");
                            }
                            
                            if let Some(_popup) = ui.begin_popup("##sidebar_chat_popup") {
                                // TODO: 
                            }

                            button(
                                ui, 
                                &std::format!("##chat_pic{}", i), 
                                [50.0, ui.content_region_avail()[1]], 
                                0.0,
                                self.theme.positive_btn_color, 
                                self.theme.positive_btn_color, 
                                self.theme.positive_btn_color, 
                            );

                            // TODO: Use tables here!
                            ui.same_line();
                            ui.text(friend.tag());
                        });
                }
            });
    }
}
impl Debug for SidebarManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SidebarManager")
            .field("buffer", &self.search_buffer)
            .finish()
    }
}