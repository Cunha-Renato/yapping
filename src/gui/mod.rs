use std::cell::OnceCell;
use yapping_core::l3gion_rust::{imgui, lg_core::{renderer::{texture::{TextureFilter, TextureFormat, TextureSpecs}, Renderer}, window::LgWindow}, StdError, UUID};

pub(crate) mod theme;
pub(crate) mod validation_gui;
pub(crate) mod sidebar_gui;
pub(crate) mod friends_page_gui;

const BORDER_RADIUS: f32 = 3.0;

#[derive(Debug, Clone, Copy)]
pub(crate) enum FontType {
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

static LOGO_PATH: &str = "assets/textures/logo.png";
static mut NEXT_WINDOW_SPECS: ([f32; 2], [f32; 2]) = ([0.0; 2], [0.0; 2]);

thread_local! {
    static FONTS: OnceCell<Fonts> = OnceCell::new();
}

pub(crate) fn init_gui(renderer: &mut Renderer, window: &LgWindow) -> Result<(), StdError> {
    // Saving Logo Image
    {
        let specs = TextureSpecs {
            tex_format: TextureFormat::RGBA,
            tex_filter: TextureFilter::NEAREST,
            ..Default::default()
        };
        
        renderer.create_texture("logo_texture", LOGO_PATH, specs)?;
    }

    // Saving the fonts
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

    renderer.set_fonts();
    std::thread::sleep(std::time::Duration::from_millis(200));

    let fonts = Fonts {
        regular_17: imgui_core.get_font_id("Roboto-Regular17").unwrap(),
        regular_24: imgui_core.get_font_id("Roboto-Regular24").unwrap(),
        bold_17: imgui_core.get_font_id("Roboto-Bold17").unwrap(),
        bold_24: imgui_core.get_font_id("Roboto-Bold24").unwrap(),
    };

    FONTS.with(|fonts_cell| fonts_cell.set(fonts).unwrap());
    
    Ok(())
}

fn window<F, R>(
    ui: &imgui::Ui,
    title: &str,
    position: [f32; 2],
    size: [f32; 2],
    min_size: [f32; 2],
    bg_color: [f32; 4],
    func: F,
) -> Option<R>
where
    F: FnOnce(&imgui::Ui) -> R
{
    let _window_bg = ui.push_style_color(imgui::StyleColor::WindowBg, bg_color);
    let _window_border = ui.push_style_var(imgui::StyleVar::WindowBorderSize(0.0));
    let _window_min_size = ui.push_style_var(imgui::StyleVar::WindowMinSize(min_size));
    let _window_padding = ui.push_style_var(imgui::StyleVar::WindowPadding([0.0; 2]));

    ui.window(title)
        .position(position, imgui::Condition::Always)
        .size(size, imgui::Condition::Always)
        .flags(imgui::WindowFlags::NO_TITLE_BAR
            | imgui::WindowFlags::NO_SCROLLBAR
            | imgui::WindowFlags::NO_SCROLL_WITH_MOUSE
            | imgui::WindowFlags::NO_MOVE
        )
        .build(|| func(&ui))
}

fn no_resize_child_window<F, R>(
    ui: &imgui::Ui,
    title: &str,
    flags: Option<imgui::WindowFlags>,
    size: [f32; 2],
    padding: [f32; 2],
    bg_color: [f32; 4],
    func: F,
) -> Option<R>
where
    F: FnOnce(&imgui::Ui) -> R
{
    let _window_bg = ui.push_style_color(imgui::StyleColor::ChildBg, bg_color);
    let _window_padding = ui.push_style_var(imgui::StyleVar::WindowPadding(padding));

    ui.child_window(title)
        .size(size)
        .flags(imgui::WindowFlags::NO_TITLE_BAR
            | imgui::WindowFlags::NO_RESIZE
            | imgui::WindowFlags::NO_MOVE
            | imgui::WindowFlags::ALWAYS_USE_WINDOW_PADDING
            | flags.unwrap_or(imgui::WindowFlags::empty())
        )
        .build(|| func(&ui))
}

fn no_resize_window<F, R>(
    ui: &imgui::Ui,
    title: &str,
    flags: Option<imgui::WindowFlags>,
    position: [f32; 2],
    size: [f32; 2],
    padding: [f32; 2],
    min_size: [f32; 2],
    bg_color: [f32; 4],
    func: F,
) -> Option<R>
where
    F: FnOnce(&imgui::Ui) -> R
{
    let _window_bg = ui.push_style_color(imgui::StyleColor::WindowBg, bg_color);
    let _window_border = ui.push_style_var(imgui::StyleVar::WindowBorderSize(0.0));
    let _window_min_size = ui.push_style_var(imgui::StyleVar::WindowMinSize(min_size));
    let _window_padding = ui.push_style_var(imgui::StyleVar::WindowPadding(padding));

    ui.window(title)
        .position(position, imgui::Condition::Always)
        .size(size, imgui::Condition::Always)
        .flags(imgui::WindowFlags::NO_TITLE_BAR
            | imgui::WindowFlags::NO_RESIZE
            | imgui::WindowFlags::NO_SCROLLBAR
            | imgui::WindowFlags::NO_SCROLL_WITH_MOUSE
            | imgui::WindowFlags::NO_MOVE
            | imgui::WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS
            | flags.unwrap_or(imgui::WindowFlags::empty())
        )
        .build(|| func(&ui))
}

fn get_logo_texture_id(renderer: &Renderer) -> Option<imgui::TextureId> {
    match renderer.get_texture(&UUID::from_string(LOGO_PATH).unwrap())
    {
        Ok(texture_ptr) => match unsafe { texture_ptr.as_ref().unwrap().gl_id() } {
            Some(gl_id) => Some(imgui::TextureId::new(gl_id as usize)),
            None => None,
        },

        Err(_) => None,
    }
}

fn show_logo(
    ui: &imgui::Ui,
    renderer: &Renderer,
    size: [f32; 2],
    pos: [f32; 2]
) {
    if let Some(logo_texture_id) = get_logo_texture_id(renderer) {
        ui.set_cursor_pos(pos);
        imgui::Image::new(logo_texture_id, size).build(ui);
    }
}

pub(crate) fn use_font(ui: &imgui::Ui, font_type: FontType) -> imgui::FontStackToken {
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
    hint: &str,
    buffer: &mut String, 
    label: &str, 
    bg_color: [f32; 4],
    text_color: [f32; 4],
    border_radius: f32,
    flags: imgui::InputTextFlags
) -> bool {
    let _bg_color_token = ui.push_style_color(imgui::StyleColor::FrameBg, bg_color);
    let _text_color_token = ui.push_style_color(imgui::StyleColor::Text, text_color);
    let _frame_rounding = ui.push_style_var(imgui::StyleVar::FrameRounding(border_radius));

    ui.input_text(label, buffer)
        .hint(hint)
        .flags(flags)
        .build()
}

fn button(
    ui: &imgui::Ui,
    label: &str,
    size: [f32; 2],
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

    let clicked = ui.button_with_size(label, size);

    idle_color.pop();
    hover_color.pop();
    active_color.pop();
    border_radius.pop();

    return clicked;
}

fn spacing(ui: &imgui::Ui, quantity: u32) {
    for _ in 0..quantity {
        ui.spacing();
    }
}

fn centered_component<R>(
    ui: &imgui::Ui,
    component_size: [f32; 2],
    component: impl FnOnce(&imgui::Ui, [f32; 2]) -> R
) -> R 
{
    let size = ui.window_size();
    let x = (size[0] - component_size[0]) / 2.0; 
    
    ui.set_cursor_pos([x, ui.cursor_pos()[1]]);
    
    component(ui, component_size)
}

pub(super) fn show_loading_gui(
    ui: &imgui::Ui, 
    renderer: &Renderer,
    position: [f32; 2], 
    size: [f32; 2],
    bg_color: [f32; 4],
) {
    no_resize_window(
        ui, 
        "Loading Window", 
        None,
        [0.0, 0.0], 
        ui.io().display_size, 
        [0.0, 0.0], 
        [0.0, 0.0], 
        bg_color, 
        |ui| {
            let content_size = ui.content_region_avail();
            let logo_size = content_size[0] / 3.0;
            
            show_logo(
                ui, 
                renderer, 
                [logo_size; 2],
                [
                    content_size[0] / 3.0, 
                    content_size[1] / 4.0 - logo_size / 2.0
                ],
            );
            
            let _font = use_font(ui, FontType::BOLD24);
            let text = "LOAGING...";
            let text_width = ui.calc_text_size(text)[0];
            ui.set_cursor_pos([ui.content_region_avail()[0] / 2.0 - text_width / 2.0, size[1] / 1.5]);
            ui.text(text);
        });
}