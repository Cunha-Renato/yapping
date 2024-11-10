use std::{collections::{hash_set::Iter, HashSet}, rc::Rc};

use yapping_core::{l3gion_rust::{imgui, lg_core::renderer::Renderer, UUID}, user::User};

use super::{theme, window, NEXT_WINDOW_SPECS};

pub(crate) struct FriendsNotificationsManager {
    theme: Rc<theme::Theme>,
    user_uuids: HashSet<UUID>,
    users: Vec<User>,
}
impl FriendsNotificationsManager {
    pub(crate) fn new(theme: Rc<theme::Theme>) -> Self {
        Self {
            theme,
            user_uuids: HashSet::default(),
            users: Vec::default(),
        }
    }

    pub(crate) fn add_notification(&mut self, sender_id: UUID) {
        self.user_uuids.insert(sender_id);
    }

    pub(crate) fn needs_querey(&mut self) -> bool {
        self.user_uuids.len() > self.users.len()
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
            self.theme.main_bg_color,
            |ui| {

            });
        
        unsafe { NEXT_WINDOW_SPECS = ([0.0; 2], [0.0; 2]); }
    }
}