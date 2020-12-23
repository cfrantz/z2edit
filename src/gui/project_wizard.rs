use std::borrow::Cow;

use imgui;
use imgui::im_str;
use imgui::ImString;

use crate::util::UTime;
use crate::zelda2::config::Config;
use crate::zelda2::import::FileResource;

const FIX_TEXT: &str = r#"Note:
  The default fixes do not change any game behavior.  The default fixes
  re-arrange certain areas of memory giving more flexibility when placing
  enemy lists and connection data."#;

#[derive(Debug)]
pub struct ProjectWizardGui {
    visible: bool,
    filename: ImString,
    configs: Vec<ImString>,
    config_sel: usize,
    pub name: ImString,
    pub rom: FileResource,
    pub fix: bool,
}

impl Default for ProjectWizardGui {
    fn default() -> Self {
        let configs = Config::keys();
        let vanilla = configs
            .binary_search_by(|s| (&**s).cmp("vanilla"))
            .unwrap_or_default();
        let configs = configs
            .iter()
            .map(|x| ImString::new(x))
            .collect::<Vec<ImString>>();

        ProjectWizardGui {
            visible: false,
            filename: ImString::new(""),
            configs: configs,
            config_sel: vanilla,
            name: ImString::new(""),
            rom: FileResource::Vanilla,
            fix: true,
        }
    }
}

impl ProjectWizardGui {
    pub fn new() -> ProjectWizardGui {
        let now = UTime::now();
        ProjectWizardGui {
            name: ImString::new(format!("Project-{}", UTime::format(now, "%Y%m%d-%H%M"))),
            ..Default::default()
        }
    }

    pub fn from_filename(filename: &str) -> ProjectWizardGui {
        let mut project = ProjectWizardGui::new();
        project.filename = ImString::new(filename);
        project.rom = FileResource::Name(filename.to_string());
        project
    }

    fn file_dialog(&self, ftype: Option<&str>) -> Option<String> {
        let result = nfd::open_file_dialog(ftype, None).expect("ProjectWizardGui::file_dialog");
        match result {
            nfd::Response::Okay(path) => Some(path),
            _ => None,
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn config(&self) -> &str {
        self.configs[self.config_sel].to_str()
    }

    pub fn draw(&mut self, ui: &imgui::Ui) -> bool {
        let mut visible = self.visible;
        let mut result = false;
        if visible {
            ui.open_popup(im_str!("Project Wizard"));
        }

        ui.popup_modal(im_str!("Project Wizard"))
            .title_bar(true)
            .opened(&mut visible)
            .build(|| {
                ui.text("New Project:");
                ui.align_text_to_frame_padding();

                ui.text("Name:");
                ui.input_text(im_str!("##name"), &mut self.name)
                    .resize_buffer(true)
                    .build();

                ui.separator();
                ui.text("ROM:");
                if ui.radio_button_bool(im_str!("Vanilla"), self.rom == FileResource::Vanilla) {
                    self.rom = FileResource::Vanilla;
                }
                if ui.radio_button_bool(im_str!("File:"), self.rom != FileResource::Vanilla) {
                    self.rom = FileResource::Name(self.filename.to_string());
                }
                ui.same_line(0.0);
                if ui
                    .input_text(im_str!("##file"), &mut self.filename)
                    .resize_buffer(true)
                    .build()
                {
                    self.rom = FileResource::Name(self.filename.to_string());
                }
                ui.same_line(0.0);
                if ui.button(im_str!("Browse##file"), [0.0, 0.0]) {
                    if let Some(filename) = self.file_dialog(Some("nes")) {
                        self.rom = FileResource::Name(filename.clone());
                        self.filename = ImString::new(filename);
                    }
                }

                ui.separator();
                ui.text("Configuration:");
                imgui::ComboBox::new(im_str!("##config")).build_simple(
                    ui,
                    &mut self.config_sel,
                    &self.configs,
                    &|x| Cow::Borrowed(&x),
                );
                ui.separator();
                ui.checkbox(im_str!("Apply default fixes to the ROM"), &mut self.fix);
                ui.text(FIX_TEXT);
                ui.separator();

                if ui.button(im_str!("  Ok  "), [0.0, 0.0]) {
                    result = true;
                    self.visible = false;
                    ui.close_current_popup();
                }
                ui.same_line(0.0);
                if ui.button(im_str!("Cancel"), [0.0, 0.0]) {
                    self.visible = false;
                    ui.close_current_popup();
                }
            });
        self.visible &= visible;
        result
    }
}
