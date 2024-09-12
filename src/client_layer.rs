use l3gion_rust::imgui;
use l3gion_rust::lg_core::event::LgKeyCode;
use l3gion_rust::sllog::info;
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
    message_buffer: String,
}
impl ClientLayer {
    pub async fn new(app_core: ApplicationCore) -> Self {
        Self {
            app_core,
            server_sender: ServerSender::new().await.unwrap(),
            message_buffer: String::default(),
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
        ui.window("Test window")
            .build(|| {
                let region_avail = ui.content_region_avail();

                if ui.input_text_multiline("##text_label", &mut self.message_buffer, [region_avail[0], 20.0])
                    .flags(
                        imgui::InputTextFlags::ALLOW_TAB_INPUT
                        | imgui::InputTextFlags::CTRL_ENTER_FOR_NEW_LINE
                        | imgui::InputTextFlags::ENTER_RETURNS_TRUE
                        | imgui::InputTextFlags::CALLBACK_RESIZE
                        | imgui::InputTextFlags::NO_HORIZONTAL_SCROLL
                    )
                    .build()
                {
                    let mut sender = self.server_sender.clone();
                    let message_buffer = self.message_buffer.clone();
                    self.message_buffer.clear();
                    tokio::spawn(async move {
                        sender.send(&message_buffer).await;
                    });
                }
            }).unwrap();
    }
}