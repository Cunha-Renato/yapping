use l3gion_rust::imgui;
use l3gion_rust::sllog::{error, info};
use l3gion_rust::{
    lg_core::{
        application::ApplicationCore,
        event::LgEvent,
        layer::Layer, 
    },
    StdError
};

use crate::server_coms::sender::ServerSender;

pub struct ClientLayer {
    app_core: ApplicationCore,
    server_sender: ServerSender,
}
impl ClientLayer {
    pub async fn new(app_core: ApplicationCore) -> Self {
        Self {
            app_core,
            server_sender: ServerSender::default(),
        }
    }
}

impl Layer for ClientLayer {
    fn debug_name(&self) -> &str {
        "ClientLayer"
    }

    fn on_attach(&mut self) -> Result<(), StdError> {
        info!("ClientLayer attached!");
        Ok(())
    }

    fn on_detach(&mut self) -> Result<(), StdError> {
        info!("ClientLayer detached!");
        Ok(())
    }

    fn on_update(&mut self) -> Result<(), StdError> {
        Ok(())
    }

    fn on_event(&mut self, event: &LgEvent) -> bool {
        match event {
            _ => ()
        }

        false
    }

    fn on_imgui(&mut self, ui: &mut imgui::Ui) {
        if !self.server_sender.connected() {
            if let Some(server_ip) = show_server_config_window_gui(ui) {
                if let Err(e) = self.server_sender.try_connect(&server_ip) {
                    error!("{:?}", e);
                }
            }
        }
        else {
            if let Some(message) = show_message_window_gui(ui) {
                let mut sender = self.server_sender.clone();

                tokio::spawn(async move {
                    if let Err(e) = sender.send(&message) {
                        error!("{:?}", e);
                    }
                });
            }
        }
    }
}

fn show_message_window_gui(ui: &mut imgui::Ui) -> Option<String> {
    let mut buffer = String::default();
    let mut result = None;

    ui.window("Message Window")
        .build(|| {
            let region_avail = ui.content_region_avail();

            if ui.input_text_multiline("##text_label", &mut buffer, [region_avail[0], 20.0])
                .flags(
                    imgui::InputTextFlags::ALLOW_TAB_INPUT
                    | imgui::InputTextFlags::CTRL_ENTER_FOR_NEW_LINE
                    | imgui::InputTextFlags::ENTER_RETURNS_TRUE
                    | imgui::InputTextFlags::CALLBACK_RESIZE
                    | imgui::InputTextFlags::NO_HORIZONTAL_SCROLL
                )
                .build()
            {
                result = Some(buffer);
            }
        });
    
    result
}

fn show_server_config_window_gui(ui: &mut imgui::Ui) -> Option<String> {
    let mut buffer = String::default();
    let mut result = None;

    ui.window("Server Config Window")
        .build(|| {
            ui.text("Server Ip: ");
            ui.same_line();
            if ui.input_text("##ip_text_label", &mut buffer)
                .enter_returns_true(true)
                .build() 
            {
                result = Some(buffer);
            }
        });
    
    result
}