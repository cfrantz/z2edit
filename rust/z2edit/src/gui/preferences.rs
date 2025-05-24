use anyhow::Result;
use pyo3::prelude::*;
use python_gui::{JsonStyle, UiContext};
use rfd::FileDialog;
use std::path::PathBuf;

use crate::app_preferences::{AppPreferences, MultiMapColor};
use crate::gui::{ErrorDialog, Visibility};
use nes::NesFile;

#[pyclass]
pub struct AppPreferencesGui {
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    config_file: PathBuf,
    pref: AppPreferences,
    vanilla_hash: String,
}

const VANILLA_HASH: &'static str =
    "ad8c0fbcf092bf84b48e69fd3964eea4ed91bfe62abc352943d537979782680c";

#[pymethods]
impl AppPreferencesGui {
    #[new]
    pub fn new(config_file: &str) -> Self {
        AppPreferencesGui {
            visible: Visibility::default(),
            error: ErrorDialog::default(),
            changed: false,
            config_file: config_file.into(),
            pref: AppPreferences::get().clone(),
            vanilla_hash: String::default(),
        }
    }

    pub fn show(&mut self) {
        self.visible = Visibility::Visible;
    }

    #[pyo3(name = "draw")]
    fn _draw(&mut self, ctx: &UiContext) {
        self.draw(ctx.ui)
    }
}

impl AppPreferencesGui {
    const BOTTOM_RESV: f32 = -40.0;
    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    pub fn draw(&mut self, ui: &imgui::Ui) {
        let mut visible = match self.check_vanilla() {
            Ok(true) => self.visible.as_bool(),
            Ok(false) | Err(_) => {
                self.visible = Visibility::Visible;
                true
            }
        };
        if !visible {
            return;
        }
        ui.window("Preferences")
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1000.0, 900.0], imgui::Condition::FirstUseEver)
            .scroll_bar(false)
            .build(|| self.editor(ui));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Palette Changed",
            "There are unsaved chagnes in the Palette Editor.\nDo you want to discard them?",
            ui,
        );
    }

    fn editor(&mut self, ui: &imgui::Ui) {
        if let Some(_tabbar) = ui.tab_bar("Preferences") {
            if let Some(_ti) = ui.tab_item("Files") {
                ui.child_window("Imgui Style")
                    .size([0.0, Self::BOTTOM_RESV])
                    .build(|| self.files_editor(ui));
            }
            if let Some(_ti) = ui.tab_item("Colors") {
                ui.child_window("Imgui Style")
                    .size([0.0, Self::BOTTOM_RESV])
                    .build(|| self.colors_editor(ui));
            }
            if let Some(_ti) = ui.tab_item("Style") {
                ui.child_window("Imgui Style")
                    .size([0.0, Self::BOTTOM_RESV])
                    .build(|| ui.show_default_style_editor());
                let current = unsafe { JsonStyle::from(ui.style()) };
                self.changed |= current != self.pref.imgui_style;
            }
        }

        ui.separator();
        if ui.button("Save") {
            self.pref.imgui_style = unsafe { JsonStyle::from(ui.style()) };
            AppPreferences::set(self.pref.clone());
            AppPreferences::save(&self.config_file);
            self.changed = false;
            self.visible = Visibility::Hidden;
        }
        ui.same_line();
        if ui.button("Cancel") {
            self.changed = false;
            self.visible = Visibility::Hidden;
        }
    }

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

    fn canonicalize(path: &mut String) {
        let target = PathBuf::from(path.as_str());
        match target.canonicalize() {
            Ok(p) => *path = p.to_string_lossy().into(),
            Err(e) => log::error!("Error canonicalizing {path:?}: {e}"),
        }
    }

    fn files_editor(&mut self, ui: &imgui::Ui) {
        ui.text("Vanilla ROM:");
        if Self::file_widget("vanilla", &mut self.pref.vanilla_rom, ui) {
            Self::canonicalize(&mut self.pref.vanilla_rom);
            self.vanilla_hash.clear();
            self.changed |= true;
        }
        match self.check_vanilla() {
            Ok(true) => ui.text_colored(Self::GREEN, "SHA256 checksum matches Vanilla ROM."),
            Ok(false) => {
                let rom = PathBuf::from(&self.pref.vanilla_rom);
                if !(rom.exists() && rom.is_file()) {
                    ui.text_colored(
                        Self::RED,
                        "Please provide the location of your unmodified Zelda II ROM.",
                    );
                } else {
                    ui.text_colored(Self::RED, "SHA256 checksum does not match Vanilla ROM.");
                }
            }
            Err(e) => ui.text_colored(Self::RED, format!("Error checking ROM: {e}")),
        }

        ui.text("\nEmulator:");
        self.changed |= Self::file_widget("emulator", &mut self.pref.emulator, ui);

        ui.text("\nFLIPS patcher:");
        self.changed |= Self::file_widget("flips_patcher", &mut self.pref.flips_patcher, ui);
    }

    fn colors_editor(&mut self, ui: &imgui::Ui) {
        self.changed |= ui
            .color_edit3_config("Application Background", &mut self.pref.background)
            .picker(true)
            .inputs(false)
            .build();

        ui.text("\nMulti-map:");
        self.changed |= ui
            .color_edit4_config(
                "Invalid Connection",
                self.pref.multimap.get_mut(&MultiMapColor::Invalid).unwrap(),
            )
            .picker(true)
            .inputs(false)
            .build();
        self.changed |= ui
            .color_edit4_config(
                "Screen 1 Connection",
                self.pref.multimap.get_mut(&MultiMapColor::Screen1).unwrap(),
            )
            .picker(true)
            .inputs(false)
            .build();
        self.changed |= ui
            .color_edit4_config(
                "Screen 2 Connection",
                self.pref.multimap.get_mut(&MultiMapColor::Screen2).unwrap(),
            )
            .picker(true)
            .inputs(false)
            .build();
        self.changed |= ui
            .color_edit4_config(
                "Screen 3 Connection",
                self.pref.multimap.get_mut(&MultiMapColor::Screen3).unwrap(),
            )
            .picker(true)
            .inputs(false)
            .build();
        self.changed |= ui
            .color_edit4_config(
                "Screen 4 Connection",
                self.pref.multimap.get_mut(&MultiMapColor::Screen4).unwrap(),
            )
            .picker(true)
            .inputs(false)
            .build();
        self.changed |= ui
            .color_edit4_config(
                "Door 1 Connection",
                self.pref.multimap.get_mut(&MultiMapColor::Door1).unwrap(),
            )
            .picker(true)
            .inputs(false)
            .build();
        self.changed |= ui
            .color_edit4_config(
                "Door 2 Connection",
                self.pref.multimap.get_mut(&MultiMapColor::Door2).unwrap(),
            )
            .picker(true)
            .inputs(false)
            .build();
        self.changed |= ui
            .color_edit4_config(
                "Door 3 Connection",
                self.pref.multimap.get_mut(&MultiMapColor::Door3).unwrap(),
            )
            .picker(true)
            .inputs(false)
            .build();
        self.changed |= ui
            .color_edit4_config(
                "Door 4 Connection",
                self.pref.multimap.get_mut(&MultiMapColor::Door4).unwrap(),
            )
            .picker(true)
            .inputs(false)
            .build();
    }

    fn check_vanilla(&mut self) -> Result<bool> {
        if self.vanilla_hash.is_empty() {
            let rom = PathBuf::from(&self.pref.vanilla_rom);
            if !(rom.exists() && rom.is_file()) {
                Ok(false)
            } else {
                let rom = NesFile::load(rom)?;
                self.vanilla_hash = rom.sha256();
                Ok(self.vanilla_hash == VANILLA_HASH)
            }
        } else {
            Ok(self.vanilla_hash == VANILLA_HASH)
        }
    }
}
