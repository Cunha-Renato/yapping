use validation_gui::gui_components;
use yapping_core::{l3gion_rust::{imgui, lg_core::{renderer::Renderer, uuid::UUID}}, user::UserCreationInfo};
use crate::{ClientMessage, gui::*};

pub(super) fn show_sign_up_gui(
    renderer: &Renderer,
    theme: &theme::Theme,
    //                  (password buffer, info).
    creation_info: &mut (String, UserCreationInfo),
    error_message: &str,
    ui: &mut imgui::Ui
) -> Option<ClientMessage>
{
    let mut result: Option<ClientMessage> = None;
    
    gui_components::window(
        ui, 
        theme, 
        "SignUpWindow",
        |ui| {
            let window_size = ui.window_size();

            gui_components::display_logo(renderer, ui);
            
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
            
            gui_components::text_input_with_title(
                ui, 
                theme, 
                "Tag:", 
                "##user_tag_sign_up", 
                &mut creation_info.1.tag, 
                imgui::InputTextFlags::empty()
            );

            gui_components::text_input_with_title(
                ui, 
                theme, 
                "Email:", 
                "##user_email_sign_up", 
                &mut creation_info.1.email, 
                imgui::InputTextFlags::empty()
            );

            gui_components::text_input_with_title(
                ui, 
                theme, 
                "Password:", 
                "##user_password_sign_up", 
                &mut creation_info.0, 
                imgui::InputTextFlags::PASSWORD
            );
            
            if let Ok(new_password) = UUID::from_string(&creation_info.0) {
                creation_info.1.password = new_password;
            };
            
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
                result = Some(ClientMessage::SIGN_UP(creation_info.1.clone()));
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
                result = Some(ClientMessage::LOGIN(UserCreationInfo::default()));
            }
            
            // Show error message
            if !error_message.is_empty() {
                let _font = use_font(ui, FontType::REGULAR17);
                spacing(ui, 5);
                let _text_color_token = ui.push_style_color(imgui::StyleColor::Text, theme.negative_actv_btn_color);
                ui.text(error_message);
            }
            
            table.end();
        });

    result
}