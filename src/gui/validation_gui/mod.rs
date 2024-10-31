use yapping_core::l3gion_rust::{imgui, lg_core::renderer::Renderer};
use crate::gui::{get_logo_texture_id, spacing, text_input, theme::Theme, use_font, BORDER_RADIUS};

pub(crate) mod validation_gui_manager;

fn display_logo(renderer: &Renderer, ui: &imgui::Ui) {
    let window_size = ui.window_size();

    super::show_logo(
        ui, 
        renderer, 
        [300.0; 2], 
        [window_size[0] / 2.0 - 150.0, window_size[1] / 4.0 - 150.0]
    );
}

fn text_input_with_title(
    ui: &imgui::Ui,
    theme: &Theme,
    title: &str, 
    label: &str,
    buffer: &mut String,
    flags: imgui::InputTextFlags
) {
    let mut _font = use_font(ui, crate::gui::FontType::BOLD24);
    ui.text(title);
    spacing(ui, 2);
    
    _font = use_font(ui, crate::gui::FontType::REGULAR24);
    ui.set_next_item_width(ui.content_region_avail()[0]);
    let _padding = ui.push_style_var(imgui::StyleVar::FramePadding([5.0, 5.0]));
    text_input(
        ui, 
        "",
        buffer, 
        label, 
        theme.input_text_bg_light, 
        [0.0, 0.0, 0.0, 1.0], 
        BORDER_RADIUS, 
        imgui::InputTextFlags::CALLBACK_RESIZE
        | flags
    );
    spacing(ui, 5);
}