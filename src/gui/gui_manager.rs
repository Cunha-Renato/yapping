use yapping_core::{client_server_coms::{Response, ServerMessage}, l3gion_rust::{imgui, lg_core::renderer::Renderer, StdError}};

use crate::server_coms::ServerCommunication;

pub(crate) trait GuiMannager {
    /// Renderer GUI
    fn on_imgui(&mut self, ui: &imgui::Ui, renderer: &Renderer);
    
    /// Change State
    fn on_update(&mut self, server_coms: &mut ServerCommunication) -> Result<(), StdError>;
    
    fn on_responded_messages(&mut self, message: &(ServerMessage, Response), server_coms: &mut ServerCommunication) -> Result<bool, StdError> {
        Ok(false)
    }
    
    fn on_received_messages(&mut self, messages: &ServerMessage) -> Result<(), StdError> {
        Ok(())
    }
}