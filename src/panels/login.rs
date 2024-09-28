use l3gion_rust::{imgui, sllog::info};

const BORDER_RADIUS: f32 = 3.0;

#[derive(Default)]
pub(crate) struct LoginGUI {
    user_tag_buffer: String,
    password_buffer: String,
}
impl LoginGUI {
    pub fn show_login_gui(&mut self, theme: &super::theme::Theme, ui: &mut imgui::Ui) {
        let mut done = false;

        let window_bg = ui.push_style_color(imgui::StyleColor::WindowBg, theme.main_bg_color);
        ui.window("Login Window")
            .position([0.0, 0.0], imgui::Condition::Always)
            .size(ui.io().display_size, imgui::Condition::Always)
            .flags(
                imgui::WindowFlags::NO_TITLE_BAR 
                | imgui::WindowFlags::NO_RESIZE 
                | imgui::WindowFlags::NO_MOVE
            )
            .build(|| {
                let mut fonts = Vec::new();
                // User Tag
                fonts.push(super::use_font(ui, super::FontType::BOLD24));
                ui.text("User Tag:");
                
                fonts.push(super::use_font(ui, super::FontType::REGULAR24));
                super::text_input(
                    ui, 
                    &mut self.user_tag_buffer, 
                    "##user_tag", 
                    theme.input_text_bg_light, 
                    [0.0, 0.0, 0.0, 1.0],
                    BORDER_RADIUS,
                    imgui::InputTextFlags::CALLBACK_RESIZE
                );
                fonts.pop();

                // Password
                ui.text("Password:");
                fonts.push(super::use_font(ui, super::FontType::REGULAR24));
                super::text_input(
                    ui, 
                    &mut self.password_buffer, 
                    "##password", 
                    theme.input_text_bg_light, 
                    [0.0, 0.0, 0.0, 1.0],
                    BORDER_RADIUS,
                    imgui::InputTextFlags::CALLBACK_RESIZE
                    | imgui::InputTextFlags::PASSWORD
                );
                
                // Buttons
                if super::button(
                    ui, 
                    "Sign Up", 
                    3.0, 
                    theme.sign_up_btn_color, 
                    theme.sign_up_btn_color, 
                    theme.sign_up_actv_btn_color, 
                ) {
                    info!("Go to Sign Up!");
                }
                ui.same_line();
                if super::button(
                    ui, 
                    "Login", 
                    3.0, 
                    theme.positive_btn_color, 
                    theme.positive_btn_color, 
                    theme.positive_actv_btn_color,
                ) {
                    info!("Go to Login!");
                }
            });
        
        window_bg.pop();
    }
}