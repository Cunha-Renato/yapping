use std::{fmt::Debug, rc::Rc};
use yapping_core::{l3gion_rust::{imgui, lg_core::renderer::Renderer}, user::User};

use super::{button, no_resize_child_window, no_resize_window, theme, window, NEXT_WINDOW_SPECS};

pub(crate) struct FriendsPageManager {
    theme: Rc<theme::Theme>,
    friends_to_display: Vec<User>,
}
impl FriendsPageManager {
    pub(crate) fn new(theme: Rc<theme::Theme>) -> Self {
        Self {
            theme,
            friends_to_display: Vec::default(),
        }
    }
    
    pub(crate) fn set_friends(&mut self, users: Vec<User>) {
        self.friends_to_display = users;
    }
    
    pub(crate) fn on_imgui(&mut self, ui: &imgui::Ui, renderer: &Renderer) {
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
                for friend in &self.friends_to_display {
                    let _window_rounding = ui.push_style_var(imgui::StyleVar::ChildRounding(3.0));
                    self.friend_cell(friend, ui);
                }
            });
        
        unsafe { NEXT_WINDOW_SPECS = ([0.0; 2], [0.0; 2]); }
    }
}
impl FriendsPageManager {
    fn friend_cell(&self, friend: &User, ui: &imgui::Ui) {
        no_resize_child_window(
            ui, 
            &std::format!("##child_window_{}", friend.uuid().to_string()),
            None, 
            [ui.content_region_avail()[0], 50.0], 
            [0.0; 2], 
            self.theme.accent_color, 
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
                    self.theme.positive_actv_btn_color, 
                    self.theme.positive_actv_btn_color, 
                    self.theme.positive_actv_btn_color, 
                );

                ui.table_next_column();

                ui.text(friend.tag());
                ui.text(std::format!("{:?}", friend.state()));
            });
    }
}
impl Debug for FriendsPageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FriendsPageManager")
            .field("friends_to_display", &self.friends_to_display)
            .finish()
    }
}