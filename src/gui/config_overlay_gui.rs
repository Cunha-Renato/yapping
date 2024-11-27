use yapping_core::{l3gion_rust::{imgui, lg_core::renderer::Renderer, lg_types::units_of_time::LgTime, AsLgTime, LgTimer, StdError}, user::User};
use crate::{client_manager::AppState, server_coms::ServerCommunication};
use super::{button, gui_manager::GuiMannager, no_resize_window, spacing, use_font, window, BORDER_RADIUS, NEXT_WINDOW_SPECS};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum ConfigState {
    MAIN,
    CHANGE_PIC,
    CHANGE_NAME,
}

pub(crate) struct ConfigOverlayGuiManager {
    app_state: AppState,
    state: ConfigState,
    show: bool,
    height: f32,
    timer: LgTimer,
    timer_init: bool,
}
impl ConfigOverlayGuiManager {
    pub(crate) fn new(app_state: AppState) -> Self {
        Self {
            app_state,
            state: ConfigState::MAIN,
            show: false,
            height: 200.0,
            timer: LgTimer::new(),
            timer_init: false,
        }
    }
    
    pub(crate) fn show(&mut self) {
        if !self.show {
            self.show = true;
            self.timer_init = false;
        }
    }
}

impl GuiMannager for ConfigOverlayGuiManager {
    fn on_imgui(&mut self, ui: &imgui::Ui, renderer: &Renderer) {
        let (mut window_pos, mut window_size) = unsafe { NEXT_WINDOW_SPECS };
        let mut new_y = window_pos[1] + (window_size[1] - self.height);

        // Animation
        if !self.timer_init {
            self.timer_init = true;
            self.timer.restart();
        }

        let time = self.timer.elapsed();
        if time < 0.5.s() {
            new_y = if !self.show {
                ease_function(new_y, window_size[1], (time.get_seconds() * 2.0) as f32)
            }
            else {
                ease_function(window_size[1], new_y, (time.get_seconds() * 2.0) as f32)
            };
        } 
        else if !self.show { return; }
        // End of Animation

        if new_y > 0.0 {
            window_pos[1] += new_y;
        }
        window_size[1] = self.height;

        no_resize_window(
            ui, 
            "config_overlay", 
            None,
            window_pos,
            window_size, 
            [10.0, 10.0],
            window_size, 
            self.app_state.theme.main_bg_color,
            |ui| match self.state {
                ConfigState::MAIN => self.show_main_gui(ui, renderer),
                ConfigState::CHANGE_PIC => todo!(),
                ConfigState::CHANGE_NAME => todo!(),
            });
    }

    fn on_update(&mut self, _server_coms: &mut ServerCommunication) -> Result<(), StdError> {
        match self.state {
            ConfigState::MAIN => (),
            ConfigState::CHANGE_PIC => todo!(),
            ConfigState::CHANGE_NAME => todo!(),
        }
                 
        Ok(())
    }
}

impl ConfigOverlayGuiManager {
    fn show_user_pic(&self, ui: &imgui::Ui, renderer: &Renderer) {
        let cursor_pos = ui.cursor_pos();
        ui.set_cursor_pos([cursor_pos[0] + 15.0, cursor_pos[1] + 15.0]);
        
        button(
            ui, 
            "##config_user_pic", 
            [150.0, 150.0], 
            BORDER_RADIUS, 
            self.app_state.theme.positive_btn_color, 
            self.app_state.theme.positive_btn_color, 
            self.app_state.theme.positive_btn_color, 
        );
    }
    
    fn show_main_gui(&mut self, ui: &imgui::Ui, renderer: &Renderer) {
        let shared_mut = self.app_state.shared_mut.borrow();
        let user = if let Some(user) = &shared_mut.user { user } else { return; };

        self.show_user_pic(ui, renderer);
        ui.same_line();

        let _table = ui.begin_table("##config_main_table", 2);

        ui.table_setup_column("##config_main_menu");
        ui.table_setup_column_with(imgui::TableColumnSetup { 
            name: "##config_main_close_btn", 
            flags: imgui::TableColumnFlags::WIDTH_FIXED, 
            init_width_or_weight: 30.0,
            ..Default::default()
        });

        ui.table_next_row();
        ui.table_next_column();
        
        let mut _font = vec![use_font(ui, super::FontType::BOLD24)];
        ui.text(user.tag());
        spacing(ui, 7);

        _font.push(use_font(ui, super::FontType::BOLD17));
        button(
            ui, 
            "Change Tag", 
            [200.0, 25.0], 
            BORDER_RADIUS,
            self.app_state.theme.accent_color, 
            self.app_state.theme.sign_up_btn_color, 
            self.app_state.theme.sign_up_btn_color, 
        ); 
        ui.spacing();
        button(
            ui, 
            "Change Picture", 
            [200.0, 25.0], 
            BORDER_RADIUS,
            self.app_state.theme.accent_color, 
            self.app_state.theme.sign_up_btn_color, 
            self.app_state.theme.sign_up_btn_color, 
        );
        ui.spacing();
        button(
            ui, 
            "Change Theme", 
            [200.0, 25.0], 
            BORDER_RADIUS,
            self.app_state.theme.accent_color, 
            self.app_state.theme.sign_up_btn_color, 
            self.app_state.theme.sign_up_btn_color, 
        ); 

        // Close button.
        ui.table_next_column();
        _font.push(use_font(ui, super::FontType::BOLD24));

        if button(
            ui, 
            "X", 
            [30.0, 30.0], 
            BORDER_RADIUS,
            self.app_state.theme.accent_color, 
            self.app_state.theme.sign_up_btn_color, 
            self.app_state.theme.sign_up_btn_color, 
        ) {
            self.show = false;
            self.timer_init = false;
            self.state = ConfigState::MAIN;
        }
    }
}

fn ease_function(a: f32, b: f32, time: f32) -> f32 {
    a + (b - a) * (-((std::f32::consts::PI * time).cos() - 1.0) / 2.0)
}