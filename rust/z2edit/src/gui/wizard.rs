use pyo3::prelude::*;
use python_gui::UiContext;
use rfd::FileDialog;

use crate::util::time::UTime;
use crate::zelda2::config::Config;
use crate::zelda2::rom::FileResource;

const FIX_TEXT: &str = r#"Note:
  The default fixes do not change any game behavior.  The default fixes
  re-arrange certain areas of memory giving more flexibility when placing
  enemy lists and connection data."#;

#[pyclass]
pub struct ProjectWizardGui {
    visible: bool,
    filename: String,
    configs: Vec<String>,
    config_sel: usize,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub rom: FileResource,
    #[pyo3(get)]
    pub fix: bool,
    #[pyo3(get, set)]
    pub done: bool,
}

impl Default for ProjectWizardGui {
    fn default() -> Self {
        let configs = Config::keys();
        let vanilla = configs
            .binary_search_by(|s| s.as_str().cmp("vanilla"))
            .unwrap_or_default();

        ProjectWizardGui {
            visible: true,
            filename: String::default(),
            configs,
            config_sel: vanilla,
            name: String::default(),
            rom: FileResource::Vanilla(),
            fix: true,
            done: false,
        }
    }
}

impl ProjectWizardGui {
    fn file_widget(id: &str, path: &mut String, ui: &imgui::Ui) -> bool {
        let mut result = ui
            .input_text(format!("##{id}"), path)
            .enter_returns_true(true)
            .build();
        ui.same_line();
        if ui.button(format!("Browse##{id}")) {
            if let Some(filename) = FileDialog::new().pick_file() {
                *path = filename.to_string_lossy().into();
                result = true;
            }
        }
        result
    }

    pub fn draw(&mut self, ui: &imgui::Ui) -> bool {
        if self.visible {
            ui.open_popup("Project Wizard");
        }
        ui.modal_popup_config("Project Wizard")
            .title_bar(true)
            .build(|| self.wizard(ui))
            .unwrap_or(false)
    }

    fn wizard(&mut self, ui: &imgui::Ui) -> bool {
        ui.text("New Project:");
        ui.align_text_to_frame_padding();

        ui.text("Name:");
        ui.input_text("##name", &mut self.name).build();

        ui.text("\nROM:");
        if ui.radio_button_bool("Vanilla", self.rom == FileResource::Vanilla()) {
            self.rom = FileResource::Vanilla();
        }
        if ui.radio_button_bool("File:", self.rom != FileResource::Vanilla()) {
            self.rom = FileResource::File(self.filename.clone());
        }
        ui.same_line();
        if Self::file_widget("file", &mut self.filename, ui) {
            self.rom = FileResource::File(self.filename.clone());
        }

        ui.text("\nConfiguration:");
        ui.combo_simple_string("##config", &mut self.config_sel, &self.configs);

        ui.separator();
        ui.checkbox("Apply defauilt fixes to the ROM", &mut self.fix);
        ui.text(FIX_TEXT);

        ui.separator();
        if ui.button("  Ok  ") {
            self.done = true;
            self.visible = false;
            ui.close_current_popup();
        }
        ui.same_line();
        if ui.button("Cancel") {
            self.visible = false;
            ui.close_current_popup();
        }
        self.done
    }
}

#[pymethods]
impl ProjectWizardGui {
    #[new]
    pub fn new() -> ProjectWizardGui {
        let now = UTime::now();
        ProjectWizardGui {
            name: format!("Project-{}", UTime::format(now, "%Y%m%d-%H%M")),
            ..Default::default()
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    #[pyo3(name = "draw")]
    fn _draw(&mut self, ctx: &UiContext) -> bool {
        self.draw(ctx.ui)
    }

    #[getter]
    fn get_config(&self) -> String {
        self.configs[self.config_sel].clone()
    }
}
