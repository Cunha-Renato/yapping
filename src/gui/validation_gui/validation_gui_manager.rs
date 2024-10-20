use yapping_core::{l3gion_rust::{imgui, lg_core::renderer::Renderer}, user::UserCreationInfo};
use crate::gui::*;

#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy)]
enum ValidationType {
    #[default]
    LOGIN,
    SIGN_UP,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct ValidationGuiManager {
    password_buffer: String,
    user_creation_info: UserCreationInfo,
    error_message: String,
    validation_type: ValidationType,
    done: bool,
}
impl ValidationGuiManager {
    pub(crate) fn is_done(&self) -> bool {
        self.done
    }

    pub(crate) fn get_creation_info(&mut self) -> UserCreationInfo {
        std::mem::take(&mut self.user_creation_info)
    }
    
    pub(crate) fn set_error_message(&mut self, message: &str) {
        self.error_message = message.to_string();
    }

    pub(crate) fn on_imgui(
        &mut self,
        ui: &mut imgui::Ui,
        renderer: &Renderer,
        theme: &theme::Theme,
    ) {
        self.done = false;
        match self.validation_type {
            ValidationType::LOGIN => self.show_login(ui, renderer, theme),
            ValidationType::SIGN_UP => self.show_sign_up(ui, renderer, theme),
        };
    }
}
impl ValidationGuiManager {
    fn show_login(
        &mut self,
        ui: &mut imgui::Ui,
        renderer: &Renderer,
        theme: &theme::Theme,
    ) {
        super::window(
            ui,
            theme,
            "LoginWindow",
            |ui| {
                let window_size = ui.window_size();

                super::display_logo(renderer, ui);
                
                // Table setup.
                let table = if let Some(table) = ui.begin_table("login_table", 2)
                {
                    table
                }
                else { return; };
                
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
                    theme, 
                    "Email:", 
                    "##user_email_login", 
                    &mut self.user_creation_info.email,
                    imgui::InputTextFlags::empty(),
                );
                
                super::text_input_with_title(
                    ui, 
                    theme, 
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
                    theme.sign_up_btn_color, 
                    theme.sign_up_btn_color, 
                    theme.sign_up_actv_btn_color, 
                ) {
                    self.validation_type = ValidationType::SIGN_UP;
                }

                ui.same_line_with_pos(ui.content_region_avail()[0] - 92.0); // No fucking idea why is 92 and not 100.
                if button(
                    ui, 
                    "Login", 
                    [100.0, 0.0],
                    3.0, 
                    theme.positive_btn_color, 
                    theme.positive_btn_color, 
                    theme.positive_actv_btn_color,
                ) {
                    if let Ok(new_password) = UUID::from_string(&self.password_buffer) {
                        self.user_creation_info.password = new_password;
                    };
                    self.password_buffer.clear();

                    self.done = true;
                }
                
                // Show error message
                if !self.error_message.is_empty() {
                    let _font = use_font(ui, FontType::REGULAR17);
                    spacing(ui, 5);
                    let _text_color_token = ui.push_style_color(imgui::StyleColor::Text, theme.negative_actv_btn_color);
                    ui.text(&self.error_message);
                }
                
                table.end();
            });
    }
    
    fn show_sign_up(
        &mut self,
        ui: &mut imgui::Ui,
        renderer: &Renderer,
        theme: &theme::Theme,
    )
    {
        super::window(
            ui, 
            theme, 
            "SignUpWindow",
            |ui| {
                let window_size = ui.window_size();
    
                super::display_logo(renderer, ui);
                
                let table = if let Some(table) = ui.begin_table("sign_up_table", 2)
                {
                    table
                }
                else { return; };
    
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
                    theme, 
                    "Tag:", 
                    "##user_tag_sign_up", 
                    &mut self.user_creation_info.tag, 
                    imgui::InputTextFlags::empty()
                );
    
                super::text_input_with_title(
                    ui, 
                    theme, 
                    "Email:", 
                    "##user_email_sign_up", 
                    &mut self.user_creation_info.email, 
                    imgui::InputTextFlags::empty()
                );
    
                super::text_input_with_title(
                    ui, 
                    theme, 
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
                    theme.sign_up_btn_color, 
                    theme.sign_up_btn_color, 
                    theme.sign_up_actv_btn_color, 
                ) {
                    if let Ok(new_password) = UUID::from_string(&self.password_buffer) {
                        self.user_creation_info.password = new_password;
                    };
                    self.password_buffer.clear();

                    self.done = true;
                }
    
                ui.same_line_with_pos(ui.content_region_avail()[0] - 92.0); // No fucking idea why is 92 and not 100.
                if button(
                    ui, 
                    "Login", 
                    [100.0, 0.0],
                    3.0, 
                    theme.positive_btn_color, 
                    theme.positive_btn_color, 
                    theme.positive_actv_btn_color,
                ) {
                    self.validation_type = ValidationType::LOGIN;
                }
                
                // Show error message
                if !self.error_message.is_empty() {
                    let _font = use_font(ui, FontType::REGULAR17);
                    spacing(ui, 5);
                    let _text_color_token = ui.push_style_color(imgui::StyleColor::Text, theme.negative_actv_btn_color);
                    ui.text(&self.error_message);
                }
                
                table.end();
            });
    }
}