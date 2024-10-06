use l3gion_rust::{imgui, lg_core::renderer::Renderer};
use crate::{user_action::UserAction, gui::*};

pub(super) fn show_login_gui(
    renderer: &Renderer,
    theme: &theme::Theme,
    mut user_tag_buffer: String,
    error_message: &str,
    ui: &mut imgui::Ui,
) -> (String, UUID, Option<UserAction>)
{
    let mut result = None;

    let mut password = UUID::from_u128(0);
    let mut password_buffer= String::default();
    let _window_bg = ui.push_style_color(imgui::StyleColor::WindowBg, theme.main_bg_color);

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
            if let Some(logo_texture_id) = get_logo_texture_id(renderer) {
                ui.set_cursor_pos([window_size[0] / 2.0 - 150.0, window_size[1] / 4.0 - 150.0]);
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
            fonts.push(use_font(ui, FontType::BOLD24));
            ui.text("User Tag:");
            spacing(ui, 2);
            
            fonts.push(use_font(ui, FontType::REGULAR24));
            ui.set_next_item_width(ui.content_region_avail()[0]);
            let mut _padding = ui.push_style_var(imgui::StyleVar::FramePadding([5.0, 5.0]));
            text_input(
                ui, 
                &mut user_tag_buffer,
                "##user_tag", 
                theme.input_text_bg_light, 
                [0.0, 0.0, 0.0, 1.0],
                BORDER_RADIUS,
                imgui::InputTextFlags::CALLBACK_RESIZE
            );

            // Password
            fonts.pop();
            spacing(ui, 5);
            ui.text("Password:");
            spacing(ui, 2);
            ui.set_next_item_width(ui.content_region_avail()[0]);
            fonts.push(use_font(ui, FontType::REGULAR24));
            text_input(
                ui, 
                &mut password_buffer, 
                "##password", 
                theme.input_text_bg_light, 
                [0.0, 0.0, 0.0, 1.0],
                BORDER_RADIUS,
                imgui::InputTextFlags::CALLBACK_RESIZE
                | imgui::InputTextFlags::PASSWORD
            );
            
            if let Ok(new_password) = UUID::from_string(&password_buffer) {
                password = new_password;
            };
            
            // Buttons
            fonts.pop();
            spacing(ui, 5);
            _padding = ui.push_style_var(imgui::StyleVar::FramePadding([7.0, 7.0]));
            if button(
                ui, 
                "Sign Up", 
                [100.0, 0.0],
                3.0, 
                theme.sign_up_btn_color, 
                theme.sign_up_btn_color, 
                theme.sign_up_actv_btn_color, 
            ) {
                result = Some(UserAction::SIGN_UP);
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
                result = Some(UserAction::LOGIN);
            }
            
            // Show error message
            if !error_message.is_empty() {
                fonts.push(use_font(ui, FontType::REGULAR17));
                spacing(ui, 5);
                let _text_color_token = ui.push_style_color(imgui::StyleColor::Text, theme.negative_actv_btn_color);
                ui.text(error_message);
            }
            
            table.end();
        });
    
    (user_tag_buffer, password, result)
}