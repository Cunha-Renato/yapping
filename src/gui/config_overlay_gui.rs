use yapping_core::{client_server_coms::{Modification, ServerMessage, ServerMessageContent}, l3gion_rust::{imgui::{self, TableColumnSetup}, lg_core::renderer::Renderer, sllog::info, AsLgTime, LgTimer, StdError}};
use crate::{client_manager::AppState, server_coms::ServerCommunication};
use super::{button, gui_manager::GuiMannager, no_resize_window, spacing, text_input, use_font, BORDER_RADIUS, NEXT_WINDOW_SPECS};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum ConfigState {
    MAIN,
    CHANGE_PIC,
    CHANGE_TAG,
}

pub(crate) struct ConfigOverlayGuiManager {
    app_state: AppState,
    state: ConfigState,
    show: bool,
    height: f32,
    timer: LgTimer,
    timer_init: bool,
    new_tag: String,
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
            new_tag: String::default(),
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
        else if !self.show { 
            self.state = ConfigState::MAIN;
            return; 
        }
        // End of Animation

        if new_y > 0.0 {
            window_pos[1] += new_y;
        }
        window_size[1] = self.height;

        let _window_bg = ui.push_style_color(imgui::StyleColor::WindowBg, self.app_state.theme.main_bg_color);
        let _window_border = ui.push_style_var(imgui::StyleVar::WindowBorderSize(0.0));
        let _window_padding = ui.push_style_var(imgui::StyleVar::WindowPadding([10.0, 10.0]));

        ui.window("config_overlay")
            .position(window_pos, imgui::Condition::Always)
            .size(window_size, imgui::Condition::Always)
            .flags(imgui::WindowFlags::NO_TITLE_BAR
                | imgui::WindowFlags::NO_RESIZE
                | imgui::WindowFlags::NO_SCROLLBAR
                | imgui::WindowFlags::NO_SCROLL_WITH_MOUSE
                | imgui::WindowFlags::NO_MOVE
            )
            .build(|| {
                match self.state {
                    ConfigState::MAIN => self.show_main_gui(ui, renderer),
                    ConfigState::CHANGE_PIC => todo!(),
                    ConfigState::CHANGE_TAG => self.show_change_tag(ui, renderer),
                }
            });
    }

    fn on_update(&mut self, server_coms: &mut ServerCommunication) -> Result<(), StdError> {
        match self.state {
            ConfigState::CHANGE_TAG => {
                if !self.new_tag.is_empty() {
                    if let Some(user) = &self.app_state.shared_mut.borrow().user {
                        server_coms.send(ServerMessage::from(ServerMessageContent::MODIFICATION(Modification::USER_TAG(user.uuid(), std::mem::take(&mut self.new_tag)))))?;
                    }
                }
            }
            _ => (),
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
    
    fn show_close_button(&mut self, ui: &imgui::Ui) {
        let _font = use_font(ui, super::FontType::BOLD24);
        ui.set_cursor_pos([ui.content_region_max()[0] - 30.0, ui.cursor_pos()[1]]);

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
        }
    }

    fn show_main_gui(&mut self, ui: &imgui::Ui, renderer: &Renderer) {
        let user_tag = if let Some(user) = &self.app_state.shared_mut.borrow().user { user.tag().to_string() } else { return; };

        self.show_user_pic(ui, renderer);
        ui.same_line();

        let _table = ui.begin_table("##config_main_table", 1);
        ui.table_setup_column_with(TableColumnSetup {
            name: "main_menus",
            flags: imgui::TableColumnFlags::WIDTH_FIXED,
            init_width_or_weight: ui.content_region_avail()[0],
            ..Default::default()
        });
        ui.table_next_row();
        ui.table_next_column();
        
        let mut _font = vec![use_font(ui, super::FontType::BOLD24)];
        ui.text(user_tag);
        ui.same_line();
        self.show_close_button(ui);
        spacing(ui, 6);

        _font.push(use_font(ui, super::FontType::BOLD17));
        if button(
            ui, 
            "Change Tag", 
            [200.0, 25.0], 
            BORDER_RADIUS,
            self.app_state.theme.accent_color, 
            self.app_state.theme.sign_up_btn_color, 
            self.app_state.theme.sign_up_btn_color, 
        ) {
            self.state = ConfigState::CHANGE_TAG;
        }

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
    }
    
    fn show_change_tag(&mut self, ui: &imgui::Ui, renderer: &Renderer) {
        let _font = use_font(ui, super::FontType::BOLD24);
        self.show_user_pic(ui, renderer);
        let mut buffer = String::new();
        
        ui.same_line();
        if text_input(
            ui, 
            "New Tag", 
            &mut buffer, 
            "##config_change_tag_input", 
            [1.0, 1.0, 1.0, 1.0], 
            [0.0, 0.0, 0.0, 1.0], 
            BORDER_RADIUS, 
            imgui::InputTextFlags::CALLBACK_RESIZE
            | imgui::InputTextFlags::ENTER_RETURNS_TRUE,
        ) {
            self.new_tag = buffer;
        }
        
        ui.same_line();
        self.show_close_button(ui);
    }
}

fn ease_function(a: f32, b: f32, time: f32) -> f32 {
    a + (b - a) * (-((std::f32::consts::PI * time).cos() - 1.0) / 2.0)
}