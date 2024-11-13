use yapping_core::{client_server_coms::{Response, ServerMessage, ServerMessageContent, Session}, l3gion_rust::{imgui, lg_core::renderer::Renderer, AsLgTime, Rfc, StdError, UUID}, user::UserCreationInfo};

use crate::{client_manager::{AppState, ForegroundState}, gui::{button, gui_manager::GuiMannager, no_resize_window, spacing, use_font, FontType}, server_coms::ServerCommunication};

#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum ValidationState {
    #[default]
    LOGIN,
    SIGN_UP,
}

pub(crate) struct ValidationGuiManager {
    app_state: AppState,
    user_creation_info: UserCreationInfo,
    validation_state: ValidationState,        
    waiting_response: UUID,
    password_buffer: String,
    error_msg: String,
    user_action: bool,
}
impl ValidationGuiManager {
    pub(crate) fn new(app_state: AppState) -> Self {
        Self {
            app_state,
            user_creation_info: UserCreationInfo::default(),
            validation_state: ValidationState::default(),
            password_buffer: String::default(),
            error_msg: String::default(),
            user_action: false,
            waiting_response: UUID::default(),
        }
    }
}

impl GuiMannager for ValidationGuiManager {
    fn on_imgui(&mut self, ui: &imgui::Ui, renderer: &Renderer) {
        self.user_action = match self.validation_state {
            ValidationState::LOGIN => self.show_login(ui, renderer),
            ValidationState::SIGN_UP => self.show_sign_up(ui, renderer),
        };
    }

    fn on_update(&mut self, server_coms: &mut ServerCommunication) -> Result<(), StdError> {
        if !self.user_action || !self.is_valid() { return Ok(()); }

        self.error_msg.clear();
        let msg_uuid = UUID::generate();
        self.waiting_response = msg_uuid;
        let info = std::mem::take(&mut self.user_creation_info);

        match server_coms.send_and_wait(1.s(), ServerMessage::new(
            msg_uuid, 
            match self.validation_state {
                ValidationState::LOGIN => ServerMessageContent::SESSION(Session::LOGIN(info)),
                ValidationState::SIGN_UP => ServerMessageContent::SESSION(Session::SIGN_UP(info)),
            }))
        {
            Ok(response) => if let Err(e) = self.handle_response(response) {
                self.error_msg = e.to_string();
            },
            Err(e) => self.error_msg = e.to_string(),
        }
        
        self.user_action = false;

        Ok(())
    }
}

impl ValidationGuiManager {
    fn handle_response(&mut self, response: ServerMessageContent) -> Result<(), StdError> {
        let response = if let ServerMessageContent::RESPONSE(response) = response { response }
        else { return Err("In ValidationGuiManager::handle_response: Got wrong message from Server!".into()); };
            
        match response {
            Response::OK_SESSION(Session::TOKEN(user)) => {
                self.app_state.shared_mut.borrow_mut().user = Some(user);
                self.app_state.shared_mut.borrow_mut().foreground_state = ForegroundState::MAIN_PAGE;
            },
            Response::Err(e) => self.error_msg = e,
            _ => self.error_msg = String::from("In ValidationGUI::on_responded_messages: Wrong response from Server!"),
        }

        Ok(())
    }

    fn is_valid(&self) -> bool {
        self.app_state.shared_mut.borrow().foreground_state == ForegroundState::VALIDATION
    }

    fn show_login(&mut self, ui: &imgui::Ui, renderer: &Renderer) -> bool
    {
        no_resize_window(
            ui,
            "LoginWindow",
            None,
            [0.0, 0.0],
            ui.io().display_size,
            [0.0, 0.0],
            [410.0, 610.0],
            self.app_state.theme.main_bg_color,
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
                    &self.app_state.theme, 
                    "Email:", 
                    "##user_email_login", 
                    &mut self.user_creation_info.email,
                    imgui::InputTextFlags::empty(),
                );
                
                super::text_input_with_title(
                    ui, 
                    &self.app_state.theme, 
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
                    self.app_state.theme.sign_up_btn_color, 
                    self.app_state.theme.sign_up_btn_color, 
                    self.app_state.theme.sign_up_actv_btn_color, 
                ) {
                    self.error_msg.clear();
                    self.password_buffer.clear();
                    self.user_creation_info = UserCreationInfo::default();
                    self.validation_state = ValidationState::SIGN_UP;
                }

                ui.same_line_with_pos(ui.content_region_avail()[0] - 100.0);
                if button(
                    ui, 
                    "Login", 
                    [100.0, 0.0],
                    3.0, 
                    self.app_state.theme.positive_btn_color, 
                    self.app_state.theme.positive_btn_color, 
                    self.app_state.theme.positive_actv_btn_color,
                ) {
                    if let Ok(new_password) = UUID::from_string(&self.password_buffer) {
                        self.user_creation_info.password = new_password;
                    };
                    self.password_buffer.clear();

                    return true;
                }
                
                // Show error message
                if !self.error_msg.is_empty() {
                    let _font = use_font(ui, FontType::REGULAR17);
                    spacing(ui, 5);
                    let _text_color_token = ui.push_style_color(imgui::StyleColor::Text, self.app_state.theme.negative_actv_btn_color);
                    ui.text(&self.error_msg);
                }
                
                table.end();
                
                false
            }).unwrap_or(false)
        }
    
    fn show_sign_up(&mut self, ui: &imgui::Ui, renderer: &Renderer) -> bool
    {
        no_resize_window(
            ui,
            "LoginWindow",
            None,
            [0.0, 0.0],
            ui.io().display_size,
            [0.0, 0.0],
            [410.0, 610.0],
            self.app_state.theme.main_bg_color,
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
                    &self.app_state.theme, 
                    "Tag:", 
                    "##user_tag_sign_up", 
                    &mut self.user_creation_info.tag, 
                    imgui::InputTextFlags::empty()
                );
    
                super::text_input_with_title(
                    ui, 
                    &self.app_state.theme, 
                    "Email:", 
                    "##user_email_sign_up", 
                    &mut self.user_creation_info.email, 
                    imgui::InputTextFlags::empty()
                );
    
                super::text_input_with_title(
                    ui, 
                    &self.app_state.theme, 
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
                    self.app_state.theme.sign_up_btn_color, 
                    self.app_state.theme.sign_up_btn_color, 
                    self.app_state.theme.sign_up_actv_btn_color, 
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
                    self.app_state.theme.positive_btn_color, 
                    self.app_state.theme.positive_btn_color, 
                    self.app_state.theme.positive_actv_btn_color,
                ) {
                    self.error_msg.clear();
                    self.password_buffer.clear();
                    self.user_creation_info = UserCreationInfo::default();
                    self.validation_state = ValidationState::LOGIN;
                }
                
                // Show error message
                if !self.error_msg.is_empty() {
                    let _font = use_font(ui, FontType::REGULAR17);
                    spacing(ui, 5);
                    let _text_color_token = ui.push_style_color(imgui::StyleColor::Text, self.app_state.theme.negative_actv_btn_color);
                    ui.text(&self.error_msg);
                }
                
                table.end();
                
                false
            }).unwrap_or(false)
    }
}