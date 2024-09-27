use l3gion_rust::{imgui::{self, sys::ImGuiDockContext_ImGuiDockContext, ConfigFlags}, lg_core::renderer::Renderer};

pub fn init_gui(renderer: &Renderer) {
    // Font
    renderer.send(l3gion_rust::lg_core::renderer::command::SendRendererCommand::SET_FONT(include_bytes!("../../resources/fonts/roboto/Roboto-Regular.ttf").to_vec(), 17.0));

    let mut core = renderer.core();
    let imgui_context = core.imgui().context();

    // imgui_context.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE
        // | imgui::ConfigFlags::VIEWPORTS_ENABLE;

    if imgui_context.io().config_flags.contains(imgui::ConfigFlags::VIEWPORTS_ENABLE) {
        imgui_context.style_mut().window_rounding = 0.0;
        unsafe {
            let dock_context = ImGuiDockContext_ImGuiDockContext();
        }
    }

    set_l3gion_theme(imgui_context);
}

pub fn config_dockspace_gui(ui: &imgui::Ui) {
    if ui.io().config_flags.contains(ConfigFlags::DOCKING_ENABLE) {
        ui.dockspace_over_main_viewport();
    }
}


fn set_l3gion_theme(context: &mut imgui::Context) {
    let orange = [0.8, 0.4, 0.21, 1.0];
    let faded_orange = [0.8, 0.5, 0.31, 1.0];
    let dark = [0.09, 0.07, 0.07, 1.0];
    let bright = [0.2, 0.18, 0.18, 1.0];

    let colors = &mut context.style_mut().colors;
    colors[imgui::StyleColor::WindowBg as usize] = [0.17647, 0.00784, 0.30980, 1.0];
    colors[imgui::StyleColor::ChildBg as usize] = orange;
    colors[imgui::StyleColor::MenuBarBg as usize] = orange;

    // Headers
    colors[imgui::StyleColor::Header as usize] = orange;
    colors[imgui::StyleColor::HeaderHovered as usize] = orange;
    colors[imgui::StyleColor::HeaderActive as usize] = orange;

    // Buttons
    colors[imgui::StyleColor::Button as usize] = dark;
    colors[imgui::StyleColor::ButtonHovered as usize] = dark;
    colors[imgui::StyleColor::ButtonActive as usize] = dark;

    // Frame BG
    colors[imgui::StyleColor::FrameBg as usize] = bright;
    colors[imgui::StyleColor::FrameBgHovered as usize] = bright;
    colors[imgui::StyleColor::FrameBgActive as usize] = bright;

    // Tabs
    colors[imgui::StyleColor::Tab as usize] = dark;
    colors[imgui::StyleColor::TabHovered as usize] = dark;
    colors[imgui::StyleColor::TabActive as usize] = dark;
    colors[imgui::StyleColor::TabUnfocused as usize] = dark;
    colors[imgui::StyleColor::TabUnfocusedActive as usize] = dark;

    // Title
    colors[imgui::StyleColor::TitleBg as usize] = dark;
    colors[imgui::StyleColor::TitleBgActive as usize] = dark;
    colors[imgui::StyleColor::TitleBgCollapsed as usize] = dark;

    // Resize
    colors[imgui::StyleColor::ResizeGrip as usize] = dark;
    colors[imgui::StyleColor::ResizeGripHovered as usize] = dark;
    colors[imgui::StyleColor::ResizeGripActive as usize] = dark;

    colors[imgui::StyleColor::Separator as usize] = dark;
    colors[imgui::StyleColor::SeparatorHovered as usize] = orange;
    colors[imgui::StyleColor::SeparatorActive as usize] = faded_orange;

    // Navigation
    colors[imgui::StyleColor::NavWindowingHighlight as usize] = orange;
    colors[imgui::StyleColor::ScrollbarGrabActive as usize] = orange;
    colors[imgui::StyleColor::NavHighlight as usize] = orange;

    // Tools
    colors[imgui::StyleColor::PopupBg as usize] = dark;
    colors[imgui::StyleColor::DockingPreview as usize] = bright;
    colors[imgui::StyleColor::DockingEmptyBg as usize] = dark;
    colors[imgui::StyleColor::CheckMark as usize] = orange;
}