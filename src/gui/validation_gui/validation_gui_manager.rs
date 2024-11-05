use std::{fmt::Debug, rc::Rc};

use yapping_core::{l3gion_rust::{imgui, lg_core::renderer::Renderer}, user::UserCreationInfo};
use crate::gui::*;

#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum ValidationAction {
    #[default]
    LOGIN,
    SIGN_UP,
}

pub(crate) struct ValidationGuiManager {
    theme: Rc<theme::Theme>,
    password_buffer: String,
    user_creation_info: UserCreationInfo,
    error_message: String,
    validation_type: ValidationAction,
}
impl ValidationGuiManager {
    pub(crate) fn new(theme: Rc<theme::Theme>) -> Self {
        Self {
            theme,
            password_buffer: String::default(),
            user_creation_info: UserCreationInfo::default(),
            error_message: String::default(),
            validation_type: ValidationAction::default(),
        }
    }

    pub(crate) fn on_imgui(
        &mut self,
        ui: &mut imgui::Ui,
        renderer: &Renderer,
        mut func: impl FnMut(ValidationAction, UserCreationInfo) -> Result<(), StdError>
    ) {
        if match self.validation_type {
            ValidationAction::LOGIN => self.show_login(ui, renderer),
            ValidationAction::SIGN_UP => self.show_sign_up(ui, renderer),
        } {
            self.error_message.clear();
            
            if let Err(e) = func(self.validation_type.clone(), std::mem::take(&mut self.user_creation_info)) {
                self.error_message = e.to_string();
            }
        }
    }
}
impl ValidationGuiManager {
    fn show_login(
        &mut self,
        ui: &mut imgui::Ui,
        renderer: &Renderer,
    ) -> bool
    {
        no_resize_window(
            ui,
            "LoginWindow",
            None,
            [0.0, 0.0],
            ui.io().display_size,
            [0.0, 0.0],
            [410.0, 610.0],
            self.theme.main_bg_color,
            |ui| {
                let window_size = ui.window_size();

                super::display_logo(renderer, ui);
                
                // Table setup.
                let table = if let Some(table) = ui.begin_table("login_table", 2)
                {
                    table
                }
                else { return false; };
                
                ui.table_setup_column_with(imgui::TableColumnSetup::<&str> { 
                    flags: imgui::TableColumnFlags::WIDTH_FIXED, 
                    init_width_or_weight: window_size[0] / 4.0, 
                    ..Default::default()
                });
                ui.table_setup_column_with(imgui::TableColumnSetup::<&str> { 
                    flags: imgui::TableColumnFlags::WIDTH_FIXED, 
                    init_width_or_weight: window_size[0] / 2.0, 
                    ..Default::default()
                });

                ui.table_next_row();
                ui.table_set_column_index(1);
                
                super::text_input_with_title(
                    ui, 
                    &self.theme, 
                    "Email:", 
                    "##user_email_login", 
                    &mut self.user_creation_info.email,
                    imgui::InputTextFlags::empty(),
                );
                
                super::text_input_with_title(
                    ui, 
                    &self.theme, 
                    "Password:", 
                    "##user_password_login", 
                    &mut self.password_buffer,
                    imgui::InputTextFlags::PASSWORD
                );
                
                // Buttons
                spacing(ui, 5);
                let _font = use_font(ui, FontType::BOLD24);
                let _padding = ui.push_style_var(imgui::StyleVar::FramePadding([7.0, 7.0]));
                if button(
                    ui, 
                    "Sign Up", 
                    [100.0, 0.0],
                    3.0, 
                    self.theme.sign_up_btn_color, 
                    self.theme.sign_up_btn_color, 
                    self.theme.sign_up_actv_btn_color, 
                ) {
                    self.error_message.clear();
                    self.password_buffer.clear();
                    self.user_creation_info = UserCreationInfo::default();
                    self.validation_type = ValidationAction::SIGN_UP;
                }

                ui.same_line_with_pos(ui.content_region_avail()[0] - 100.0);
                if button(
                    ui, 
                    "Login", 
                    [100.0, 0.0],
                    3.0, 
                    self.theme.positive_btn_color, 
                    self.theme.positive_btn_color, 
                    self.theme.positive_actv_btn_color,
                ) {
                    if let Ok(new_password) = UUID::from_string(&self.password_buffer) {
                        self.user_creation_info.password = new_password;
                    };
                    self.password_buffer.clear();

                    return true;
                }
                
                // Show error message
                if !self.error_message.is_empty() {
                    let _font = use_font(ui, FontType::REGULAR17);
                    spacing(ui, 5);
                    let _text_color_token = ui.push_style_color(imgui::StyleColor::Text, self.theme.negative_actv_btn_color);
                    ui.text(&self.error_message);
                }
                
                table.end();
                
                false
            }).unwrap_or(false)
        }
    
    fn show_sign_up(
        &mut self,
        ui: &mut imgui::Ui,
        renderer: &Renderer,
    ) -> bool
    {
        no_resize_window(
            ui,
            "LoginWindow",
            None,
            [0.0, 0.0],
            ui.io().display_size,
            [0.0, 0.0],
            [410.0, 610.0],
            self.theme.main_bg_color,
            |ui| {
                let window_size = ui.window_size();
    
                super::display_logo(renderer, ui);
                
                let table = if let Some(table) = ui.begin_table("sign_up_table", 2)
                {
                    table
                }
                else { return false; };
    
                ui.table_setup_column_with(imgui::TableColumnSetup::<&str> { 
                    flags: imgui::TableColumnFlags::WIDTH_FIXED, 
                    init_width_or_weight: window_size[0] / 4.0, 
                    ..Default::default()
                });
                ui.table_setup_column_with(imgui::TableColumnSetup::<&str> { 
                    flags: imgui::TableColumnFlags::WIDTH_FIXED, 
                    init_width_or_weight: window_size[0] / 2.0, 
                    ..Default::default()
                });
    
                ui.table_next_row();
                ui.table_set_column_index(1);
                
                super::text_input_with_title(
                    ui, 
                    &self.theme, 
                    "Tag:", 
                    "##user_tag_sign_up", 
                    &mut self.user_creation_info.tag, 
                    imgui::InputTextFlags::empty()
                );
    
                super::text_input_with_title(
                    ui, 
                    &self.theme, 
                    "Email:", 
                    "##user_email_sign_up", 
                    &mut self.user_creation_info.email, 
                    imgui::InputTextFlags::empty()
                );
    
                super::text_input_with_title(
                    ui, 
                    &self.theme, 
                    "Password:", 
                    "##user_password_sign_up", 
                    &mut self.password_buffer,
                    imgui::InputTextFlags::PASSWORD
                );
                
                // Buttons
                spacing(ui, 5);
                let _font = use_font(ui, FontType::BOLD24);
                let _padding = ui.push_style_var(imgui::StyleVar::FramePadding([7.0, 7.0]));
                if button(
                    ui, 
                    "Sign Up", 
                    [100.0, 0.0],
                    3.0, 
                    self.theme.sign_up_btn_color, 
                    self.theme.sign_up_btn_color, 
                    self.theme.sign_up_actv_btn_color, 
                ) {
                    if let Ok(new_password) = UUID::from_string(&self.password_buffer) {
                        self.user_creation_info.password = new_password;
                    }
                    self.password_buffer.clear();

                    return true;
                }
    
                ui.same_line_with_pos(ui.content_region_avail()[0] - 100.0);
                if button(
                    ui, 
                    "Login", 
                    [100.0, 0.0],
                    3.0, 
                    self.theme.positive_btn_color, 
                    self.theme.positive_btn_color, 
                    self.theme.positive_actv_btn_color,
                ) {
                    self.error_message.clear();
                    self.password_buffer.clear();
                    self.user_creation_info = UserCreationInfo::default();
                    self.validation_type = ValidationAction::LOGIN;
                }
                
                // Show error message
                if !self.error_message.is_empty() {
                    let _font = use_font(ui, FontType::REGULAR17);
                    spacing(ui, 5);
                    let _text_color_token = ui.push_style_color(imgui::StyleColor::Text, self.theme.negative_actv_btn_color);
                    ui.text(&self.error_message);
                }
                
                table.end();
                
                false
            }).unwrap_or(false)
    }
}
impl Debug for ValidationGuiManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValidationGuiManager")
            .field("password_buffer", &self.password_buffer)
            .field("user_creation_info", &self.user_creation_info)
            .field("error_message", &self.error_message)
            .field("validation_type", &self.validation_type)
            .finish()
    }
}