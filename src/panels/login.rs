use l3gion_rust::{imgui, lg_core::renderer::Renderer, sllog::info, StdError};

const BORDER_RADIUS: f32 = 3.0;

#[derive(Default)]
pub(crate) struct LoginGUI {
    user_tag_buffer: String,
    password_buffer: String,
}
impl LoginGUI {
    pub(crate) fn show_login_gui(
        &mut self, 
        renderer: &Renderer,
        theme: &super::theme::Theme,
        ui: &mut imgui::Ui
    ) {
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
                let window_size = ui.window_size();

                // Logo
                if let Some(logo_texture_id) = super::get_logo_texture_id(renderer) {
                    ui.set_cursor_pos([window_size[0] / 2.0 - 150.0, window_size[1] / 3.0 - 150.0]);
                    imgui::Image::new(logo_texture_id, [300.0, 300.0]).build(ui);
                }

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

                let mut fonts = Vec::new();

                // User Tag
                fonts.push(super::use_font(ui, super::FontType::BOLD24));
                ui.text("User Tag:");
                super::spacing(ui, 2);
                
                fonts.push(super::use_font(ui, super::FontType::REGULAR24));
                ui.set_next_item_width(ui.content_region_avail()[0]);
                let mut padding = ui.push_style_var(imgui::StyleVar::FramePadding([5.0, 5.0]));
                super::text_input(
                    ui, 
                    &mut self.user_tag_buffer, 
                    "##user_tag", 
                    theme.input_text_bg_light, 
                    [0.0, 0.0, 0.0, 1.0],
                    BORDER_RADIUS,
                    imgui::InputTextFlags::CALLBACK_RESIZE
                );

                // Password
                fonts.pop();
                super::spacing(ui, 5);
                ui.text("Password:");
                super::spacing(ui, 2);
                ui.set_next_item_width(ui.content_region_avail()[0]);
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
                fonts.pop();
                super::spacing(ui, 5);
                let padding = ui.push_style_var(imgui::StyleVar::FramePadding([7.0, 7.0]));
                if super::button(
                    ui, 
                    "Sign Up", 
                    [100.0, 0.0],
                    3.0, 
                    theme.sign_up_btn_color, 
                    theme.sign_up_btn_color, 
                    theme.sign_up_actv_btn_color, 
                ) {
                    info!("Go to Sign Up!");
                }

                ui.same_line_with_pos(ui.content_region_avail()[0] - 92.0); // No fucking idea why is 92 and not 100.
                if super::button(
                    ui, 
                    "Login", 
                    [100.0, 0.0],
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