use std::rc::Rc;

use yapping_core::{l3gion_rust::{imgui, lg_core::{input::LgInput, renderer::Renderer}, sllog::warn}, user::User};
use super::{button, centered_component, no_resize_child_window, no_resize_window, spacing, text_input, theme, use_font, NEXT_WINDOW_SPECS};

pub(crate) struct SidebarManager {
    theme: Rc<theme::Theme>,
    friend_tag: String,
}
impl SidebarManager {
    pub(crate) fn new(theme: Rc<theme::Theme>) -> Self {
        Self { 
            theme, 
            friend_tag: String::default()
        }
    }

    pub(crate) fn on_imgui(&mut self, ui: &imgui::Ui, renderer: &Renderer, friends: &[User]) {
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
                }
                
                // Friends Search
                spacing(ui, 5);
                _fonts.push(use_font(ui, super::FontType::REGULAR24));
                ui.text("Friends");

                spacing(ui, 1);
                _fonts.push(use_font(ui, super::FontType::REGULAR17));
                if self.show_friend_search(ui) {
                    warn!("Search for User: {}", self.friend_tag);
                    self.friend_tag.clear();
                }
                
                // Friend List
                self.show_friend_list(ui, friends);
            });
        
        unsafe { NEXT_WINDOW_SPECS = ([window_size[0], 0.0], [ui.io().display_size[0] - window_size[0], ui.io().display_size[1]]) };
        
        test(ui, renderer);
    }
}
impl SidebarManager {
    fn show_friend_search(&mut self, ui: &imgui::Ui) -> bool {
        let _window_rounding = ui.push_style_var(imgui::StyleVar::ChildRounding(5.0));
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
                    3.0, 
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
            [0.0, 0.0, 0.0, 1.0],
            // self.theme.left_panel_bg_color, 
            |ui| {
                let _window_rounding = ui.push_style_var(imgui::StyleVar::ChildRounding(5.0));
                for (i, friend) in friends.iter().enumerate() {
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

pub(crate) fn test(ui: &imgui::Ui, renderer: &Renderer) {
    let (window_pos, window_size) = unsafe { NEXT_WINDOW_SPECS };

    no_resize_window(
        ui, 
        "test", 
        None,
        window_pos,
        window_size, 
        [0.0, 0.0],
        window_size, 
        [0.3, 0.3, 0.3, 1.0],
        |ui| {
            ui.text("TEST");
        });
    
    unsafe { NEXT_WINDOW_SPECS = ([0.0; 2], [0.0; 2]); }
}