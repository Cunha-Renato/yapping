use std::{fmt::Debug, rc::Rc};
use yapping_core::{l3gion_rust::{imgui, lg_core::renderer::Renderer, UUID}, user::User};

use super::{button, no_resize_child_window, no_resize_window, theme, window, BORDER_RADIUS, NEXT_WINDOW_SPECS};

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
    
    pub(crate) fn on_imgui(&mut self, ui: &imgui::Ui, renderer: &Renderer) -> Option<UUID> {
        let (window_pos, window_size) = unsafe { NEXT_WINDOW_SPECS };

        let result = window(
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
                    if self.friend_cell(friend, ui) {
                        return Some(friend.uuid());
                    }
                }
                
                None
            })
            .unwrap_or(None);
        
        unsafe { NEXT_WINDOW_SPECS = ([0.0; 2], [0.0; 2]); }

        result
    }
}
impl FriendsPageManager {
    fn friend_cell(&self, friend: &User, ui: &imgui::Ui) -> bool {
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
                
                if ui.is_window_hovered() && ui.is_mouse_clicked(imgui::MouseButton::Right) {
                    ui.open_popup("##sidebar_friend_popup");
                }
                
                if let Some(_popup) = ui.begin_popup("##sidebar_friend_popup") {
                    // TODO: 
                    if button(
                        ui, 
                        "Add Friend", 
                        [30.0, 15.0], 
                        BORDER_RADIUS, 
                        self.theme.accent_color, 
                        self.theme.main_bg_color, 
                        self.theme.main_bg_color, 
                    ) {
                        return true;
                    }
                }
                
                false
            })
            .unwrap_or(false)
    }
}
impl Debug for FriendsPageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FriendsPageManager")
            .field("friends_to_display", &self.friends_to_display)
            .finish()
    }
}