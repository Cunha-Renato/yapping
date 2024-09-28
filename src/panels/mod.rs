use std::cell::OnceCell;
use l3gion_rust::{imgui, lg_core::renderer::{command::SendRendererCommand, Renderer}};

#[derive(Debug, Clone, Copy)]
enum FontType {
    REGULAR17,
    REGULAR24,
    BOLD17,
    BOLD24,
}

#[derive(Debug, Copy, Clone)]
struct Fonts {
    regular_17: imgui::FontId,
    regular_24: imgui::FontId,
    bold_17: imgui::FontId,
    bold_24: imgui::FontId,
}

thread_local! {
    static FONTS: OnceCell<Fonts> = OnceCell::new();
}

pub mod theme;
pub mod login;

pub fn init_gui(renderer: &mut Renderer) {
    let mut core = renderer.core();
    let imgui_core = core.imgui();
    
    imgui_core.insert_font_id(vec![
        (
            String::from("Roboto-Regular17"),
            imgui::FontSource::TtfData { 
                data: include_bytes!("../../resources/fonts/roboto/Roboto-Regular.ttf"),
                size_pixels: 17.0,
                config: None 
            }
        ),
        (
            String::from("Roboto-Regular24"),
            imgui::FontSource::TtfData { 
                data: include_bytes!("../../resources/fonts/roboto/Roboto-Regular.ttf"),
                size_pixels: 24.0,
                config: None 
            }
        ),
        (
            String::from("Roboto-Bold17"),
            imgui::FontSource::TtfData { 
                data: include_bytes!("../../resources/fonts/roboto/Roboto-Bold.ttf"),
                size_pixels: 24.0,
                config: None 
            }
        ),
        (
            String::from("Roboto-Bold24"),
            imgui::FontSource::TtfData { 
                data: include_bytes!("../../resources/fonts/roboto/Roboto-Bold.ttf"),
                size_pixels: 24.0,
                config: None 
            }
        ),
    ]);

    renderer.send_and_wait(SendRendererCommand::SET_FONTS(), std::time::Duration::from_millis(500));

    let fonts = Fonts {
        regular_17: imgui_core.get_font_id("Roboto-Regular17").unwrap(),
        regular_24: imgui_core.get_font_id("Roboto-Regular24").unwrap(),
        bold_17: imgui_core.get_font_id("Roboto-Bold17").unwrap(),
        bold_24: imgui_core.get_font_id("Roboto-Bold24").unwrap(),
    };

    FONTS.with(|fonts_cell| fonts_cell.set(fonts).unwrap());
}

fn use_font(ui: &imgui::Ui, font_type: FontType) -> imgui::FontStackToken {
    FONTS.with(|font| {
        let font = font.get().unwrap();
        
        let to_use = match font_type {
            FontType::REGULAR17 => font.regular_17,
            FontType::REGULAR24 => font.regular_24,
            FontType::BOLD17 => font.bold_17,
            FontType::BOLD24 => font.bold_24,
        };
        
        ui.push_font(to_use)
    })
}

fn text_input(
    ui: &imgui::Ui, 
    buffer: &mut String, 
    label: &str, 
    bg_color: [f32; 4],
    text_color: [f32; 4],
    border_radius: f32,
    flags: imgui::InputTextFlags
) {
    let bg_color_token = ui.push_style_color(imgui::StyleColor::FrameBg, bg_color);
    let text_color_token = ui.push_style_color(imgui::StyleColor::Text, text_color);
    let frame_rounding = ui.push_style_var(imgui::StyleVar::FrameRounding(border_radius));

    ui.input_text(label, buffer)
        .flags(flags)
        .build();

    bg_color_token.end();
    text_color_token.end();
    frame_rounding.end();
}

fn button(
    ui: &imgui::Ui,
    label: &str,
    border_radius: f32,
    idle_color: [f32; 4],
    hover_color: [f32; 4],
    active_color: [f32; 4]
) -> bool 
{
    let idle_color = ui.push_style_color(imgui::StyleColor::Button, idle_color);
    let hover_color = ui.push_style_color(imgui::StyleColor::ButtonHovered, hover_color);
    let active_color = ui.push_style_color(imgui::StyleColor::ButtonActive, active_color);
    let border_radius = ui.push_style_var(imgui::StyleVar::FrameRounding(border_radius));

    let clicked = ui.button(label);

    idle_color.pop();
    hover_color.pop();
    active_color.pop();
    border_radius.pop();

    return clicked;
}