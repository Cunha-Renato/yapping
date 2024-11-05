use std::{fmt::Debug, rc::Rc};
use yapping_core::l3gion_rust::{imgui, lg_core::renderer::Renderer};

use super::{no_resize_window, theme, NEXT_WINDOW_SPECS};

pub(crate) struct FriendsPageManager {
    theme: Rc<theme::Theme>,
}
impl FriendsPageManager {
    pub(crate) fn new(theme: Rc<theme::Theme>) -> Self {
        Self {
            theme,
        }
    }
    
    pub(crate) fn on_imgui(&mut self, ui: &imgui::Ui, renderer: &Renderer) {
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
}
impl Debug for FriendsPageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FriendsPageManager")
            .finish()
    }
}