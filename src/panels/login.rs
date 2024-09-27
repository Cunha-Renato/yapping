use l3gion_rust::imgui;

#[derive(Default)]
pub struct LoginGUI {
    user_tag_buffer: String,
    password_buffer: String,
}
impl LoginGUI {
    pub fn show_login_gui(&mut self, ui: &mut imgui::Ui) {
        let mut done = false;

        ui.window("Login Window")
            .position([0.0, 0.0], imgui::Condition::Always)
            .size(ui.io().display_size, imgui::Condition::Always)
            .flags(
                imgui::WindowFlags::NO_TITLE_BAR 
                | imgui::WindowFlags::NO_RESIZE 
                | imgui::WindowFlags::NO_MOVE
            )
            .build(|| {
                ui.text("User Tag:");
                super::text_input(
                    ui, 
                    &mut self.user_tag_buffer, 
                    "##user_tag", 
                    [1.0, 1.0, 1.0, 1.0], 
                    [0.0, 0.0, 0.0, 1.0],
                    imgui::InputTextFlags::CALLBACK_RESIZE
                );

                ui.text("Password:");
                super::text_input(
                    ui, 
                    &mut self.password_buffer, 
                    "##password", 
                    [1.0, 1.0, 1.0, 1.0], 
                    [0.0, 0.0, 0.0, 1.0],
                    imgui::InputTextFlags::CALLBACK_RESIZE
                    | imgui::InputTextFlags::PASSWORD
                );
                
                ui.button("Sign in");
                ui.same_line();
                ui.button("Log in");
            });
    }
}