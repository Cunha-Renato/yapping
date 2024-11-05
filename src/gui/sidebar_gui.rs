use std::{fmt::Debug, rc::Rc};

use yapping_core::{l3gion_rust::{imgui, lg_core::{input::LgInput, renderer::Renderer}, sllog::warn}, user::User};
use super::{button, centered_component, no_resize_child_window, no_resize_window, spacing, text_input, theme, use_font, BORDER_RADIUS, NEXT_WINDOW_SPECS};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub(crate) enum SidebarAction {
    FIND_NEW_FRIEND(String),
}

#[derive(Debug, Clone)]
enum SidebarState {
    FRIENDS,
    CHATS
}

pub(crate) struct SidebarManager {
    state: SidebarState,
    theme: Rc<theme::Theme>,
    friend_tag: String,
}
impl SidebarManager {
    pub(crate) fn new(theme: Rc<theme::Theme>) -> Self {
        Self { 
            state: SidebarState::FRIENDS,
            theme, 
            friend_tag: String::default()
        }
    }

    pub(crate) fn on_imgui(
        &mut self, 
        ui: &imgui::Ui, 
        renderer: &Renderer, 
        friends: &[User],
        mut func: impl FnMut(SidebarAction)
    ) {
        let window_size = [200.0, ui.io().display_size[1]];

        if let Some(action) = match self.state {
            SidebarState::FRIENDS => self.show_friends_sidebar(renderer, friends, ui),
            SidebarState::CHATS => todo!(),
        } {
            func(action);
        }
        
        unsafe { NEXT_WINDOW_SPECS = ([window_size[0], 0.0], [ui.io().display_size[0] - window_size[0], ui.io().display_size[1]]) };
    }
}
impl SidebarManager {
    fn show_friends_sidebar(
        &mut self, 
        renderer: &Renderer, 
        friends: &[User],
        ui: &imgui::Ui
    ) -> Option<SidebarAction>
    {
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
                let mut _fonts = vec![use_font(ui, super::FontType::BOLD24)];

                let mut size = ui.content_region_avail();
                size[1] = 50.0;
                
                if centered_component(ui, size, |ui, c_size| button(
                    ui, 
                    "Chats", 
                    c_size, 
                    50.0, 
                    self.theme.accent_color, 
                    self.theme.main_bg_color, 
                    self.theme.accent_color, 
                )) {
                   warn!("GOTO CHATS!");
                   // TODO: self.state = SidebarState::CHATS;
                }
                
                // Friends Search
                spacing(ui, 5);
                _fonts.push(use_font(ui, super::FontType::REGULAR24));
                ui.text("Friends");

                spacing(ui, 1);
                _fonts.push(use_font(ui, super::FontType::REGULAR17));
                if self.show_friend_search(ui) {
                    return Some(SidebarAction::FIND_NEW_FRIEND(std::mem::take(&mut self.friend_tag)));
                }
                
                self.show_friend_list(ui, friends);
                
                spacing(ui, 1);
                _fonts.push(use_font(ui, super::FontType::BOLD17));
                if self.show_friend_requests_btn(ui) {
                    warn!("GOTO FIREND_REQUESTS");
                }

                None
            })
            .unwrap_or(None)
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

    fn show_friend_search(&mut self, ui: &imgui::Ui) -> bool {
        let _window_rounding = ui.push_style_var(imgui::StyleVar::ChildRounding(BORDER_RADIUS));
        no_resize_child_window(
            ui, 
            "friend_search", 
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
                    "User Tag",
                    &mut self.friend_tag, 
                    "##friends_search", 
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
                    .filter(|(_, friend)| friend.tag().to_lowercase().contains(&self.friend_tag.to_lowercase()))
                {
                    no_resize_child_window(
                        ui, 
                        &std::format!("friend_{}", i), 
                        None, 
                        [ui.content_region_avail()[0], 50.0], 
                        [0.0, 0.0], 
                        self.theme.accent_color, 
                        |ui| {
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
}
impl Debug for SidebarManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SidebarManager")
            .field("friend_tag", &self.friend_tag)
            .finish()
    }
}