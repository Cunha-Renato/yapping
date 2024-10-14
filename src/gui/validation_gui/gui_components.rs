use yapping_core::l3gion_rust::{imgui, lg_core::renderer::Renderer};
use crate::gui::{get_logo_texture_id, spacing, text_input, theme::Theme, use_font, BORDER_RADIUS};

pub(super) fn window<F, R>(
    ui: &mut imgui::Ui,
    theme: &Theme,
    title: &str,
    func: F,
) -> Option<R>
where
    F: FnOnce(&imgui::Ui) -> R
{
    let _window_bg = ui.push_style_color(imgui::StyleColor::WindowBg, theme.main_bg_color);
    ui.window(title)
        .position([0.0, 0.0], imgui::Condition::Always)
        .size(ui.io().display_size, imgui::Condition::Always)
        .flags(imgui::WindowFlags::NO_TITLE_BAR
            | imgui::WindowFlags::NO_RESIZE
            | imgui::WindowFlags::NO_MOVE
        )
        .build(|| func(&ui))
}

pub(super) fn display_logo(renderer: &Renderer, ui: &imgui::Ui) {
    let window_size = ui.window_size();

    if let Some(logo_texture_id) = get_logo_texture_id(renderer) {
        ui.set_cursor_pos([window_size[0] / 2.0 - 150.0, window_size[1] / 4.0 - 150.0]);
        imgui::Image::new(logo_texture_id, [300.0, 300.0]).build(ui);
    }
}

pub(super) fn text_input_with_title(
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