use imgui;
use imgui::im_str;

#[derive(Clone, Debug)]
pub struct Preferences {
    pub visible: bool,
    pub background: [f32; 3],
}

impl Preferences {
    pub fn new() -> Self {
        Preferences {
            visible: false,
            background: [0f32, 0f32, 0.42f32],
        }
    }

    pub fn draw(&mut self, ui: &imgui::Ui) {
        let mut visible = self.visible;
        if !visible {
            return;
        }
        imgui::Window::new(im_str!("Preferences"))
            .opened(&mut visible)
            .build(&ui, || {
                ui.text(format!(
                    "Instantaneous FPS: {:>6.02}", 1.0 / ui.io().delta_time));

                imgui::ColorEdit::new(im_str!("Background"), &mut self.background)
                    .alpha(false)
                    .inputs(false)
                    .picker(true)
                    .build(&ui);
            });
        self.visible = visible;
    }
}
