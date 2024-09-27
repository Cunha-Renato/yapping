use l3gion_rust::imgui;

pub mod config;
pub mod theme;
pub mod login;

const BORDER_RADIUS: f32 = 3.0;

fn text_input(
    ui: &imgui::Ui, 
    buffer: &mut String, 
    label: &str, 
    bg_color: [f32; 4],
    text_color: [f32; 4],
    flags: imgui::InputTextFlags
) {
    let bg_color_token = ui.push_style_color(imgui::StyleColor::FrameBg, bg_color);
    let text_color_token = ui.push_style_color(imgui::StyleColor::Text, text_color);
    let frame_rounding = ui.push_style_var(imgui::StyleVar::FrameRounding(BORDER_RADIUS));

    ui.input_text(label, buffer)
        .flags(flags)
        .build();

    bg_color_token.end();
    text_color_token.end();
    frame_rounding.end();
}