#[derive(Debug, Clone, Copy)]
pub(crate) struct Theme {
    pub(crate) font_color: [f32; 4],
    pub(crate) main_bg_color: [f32; 4],
    pub(crate) left_panel_bg_color: [f32; 4],
    pub(crate) mini_pop_up_color: [f32; 4],
    pub(crate) positive_btn_color: [f32; 4],
    pub(crate) positive_actv_btn_color: [f32; 4],
    pub(crate) negative_btn_color: [f32; 4],
    pub(crate) negative_actv_btn_color: [f32; 4],
    pub(crate) sign_up_btn_color: [f32; 4],
    pub(crate) sign_up_actv_btn_color: [f32; 4],
    pub(crate) accent_color: [f32; 4],
    pub(crate) input_text_bg_light: [f32; 4],
    pub(crate) input_text_bg_dark: [f32; 4],
}

pub(crate) static MAIN_THEME: Theme = Theme {
    font_color:                 [1.0, 1.0, 1.0, 1.0],
    main_bg_color:              [0.176, 0.0078, 0.3098, 1.0],
    left_panel_bg_color:        [0.2588, 0.0, 0.4666, 1.0],
    mini_pop_up_color:          [0.0823, 0.0, 0.1490, 1.0],
    positive_btn_color:         [0.1372, 0.6470, 0.3490, 1.0],
    positive_actv_btn_color:    [0.1372, 0.7470, 0.3490, 1.0],
    negative_btn_color:         [0.6274, 0.1568, 0.1568, 1.0],
    negative_actv_btn_color:    [0.7274, 0.1568, 0.1568, 1.0],
    sign_up_btn_color:          [0.5019, 0.0, 1.0, 1.0],
    sign_up_actv_btn_color:     [0.6019, 0.0, 1.0, 1.0],
    accent_color:               [0.2901, 0.0039, 0.6509, 1.0],
    input_text_bg_light:        [1.0, 1.0, 1.0, 1.0],
    input_text_bg_dark:         [0.8588, 0.8588, 0.8588, 1.0],
};