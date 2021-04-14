use imgui;
use imgui::im_str;
use pyo3::prelude::*;
use ron;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::errors::*;
use crate::gui::app_context::AppContext;
use crate::gui::Visibility;
use crate::nes::Buffer;

#[derive(Clone, Debug, SmartDefault, Serialize, Deserialize)]
pub struct Preferences {
    #[serde(default)]
    #[default = ""]
    pub vanilla_rom: PathBuf,
    #[serde(default)]
    #[default(_code = "[0.0, 0.1, 0.25]")]
    pub background: [f32; 3],
    #[serde(default)]
    #[default = "fceux"]
    pub emulator: String,
    #[serde(default)]
    pub multimap: Vec<[f32; 4]>,
}

const MULTIMAP_COLORS: [[f32; 4]; 9] = [
    [0.8, 0.9, 0.0, 0.9],
    [0.0, 1.0, 0.0, 0.9],
    [0.0, 0.0, 1.0, 0.9],
    [1.0, 0.0, 0.0, 0.9],
    [0.9, 0.5, 0.0, 0.9],
    [0.9, 0.5, 0.0, 0.9],
    [0.9, 0.5, 0.0, 0.9],
    [0.9, 0.5, 0.0, 0.9],
    [0.4, 0.4, 0.4, 0.5],
];
const MULTIMAP_NAMES: [&'static str; 9] = [
    "Screen 1", "Screen 2", "Screen 3", "Screen 4", "Door 1", "Door 2", "Door 3", "Door 4",
    "Invalid",
];

impl Preferences {
    pub fn new() -> Preferences {
        let mut pref = Preferences::default();
        pref.check_colors();
        pref
    }

    pub fn from_reader(r: impl Read) -> Result<Self> {
        let mut pref: Self = ron::de::from_reader(r)?;
        pref.check_colors();
        Ok(pref)
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        Preferences::from_reader(&file)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let pretty = ron::ser::PrettyConfig::new();
        let val = ron::ser::to_string_pretty(self, pretty).unwrap();
        std::fs::write(path, &val)?;
        Ok(())
    }

    fn check_colors(&mut self) {
        for i in self.multimap.len()..MULTIMAP_COLORS.len() {
            self.multimap.push(MULTIMAP_COLORS[i]);
        }
    }
}

const VANILLA_HASH: &'static str =
    "ad8c0fbcf092bf84b48e69fd3964eea4ed91bfe62abc352943d537979782680c";

#[pyclass]
#[derive(Debug, Default)]
pub struct PreferencesGui {
    pub visible: Visibility,
    pub changed: bool,
    pub vanilla_hash: String,
}

impl PreferencesGui {
    pub fn check_vanilla(&mut self) -> Result<bool> {
        let pref = AppContext::pref();
        if self.vanilla_hash.is_empty() {
            if !(pref.vanilla_rom.exists() && pref.vanilla_rom.is_file()) {
                Ok(false)
            } else {
                let buffer = Buffer::from_file(&pref.vanilla_rom, None)?;
                self.vanilla_hash = buffer.sha256();
                Ok(self.vanilla_hash == VANILLA_HASH)
            }
        } else {
            Ok(self.vanilla_hash == VANILLA_HASH)
        }
    }

    pub fn show(&mut self) {
        self.visible = Visibility::Visible;
    }

    fn file_dialog(&self, ftype: Option<&str>) -> Option<String> {
        let result = nfd::open_file_dialog(ftype, None).expect("PreferencesGui::file_dialog");
        match result {
            nfd::Response::Okay(path) => Some(path),
            _ => None,
        }
    }
    fn draw_connection_colors(
        &self,
        pref: &mut Preferences,
        start: usize,
        end: usize,
        ui: &imgui::Ui,
    ) -> bool {
        let mut changed = false;
        ui.group(|| {
            for i in start..end {
                changed |= imgui::ColorEdit::new(
                    &im_str!("{} Connection Color", MULTIMAP_NAMES[i]),
                    &mut pref.multimap[i],
                )
                .alpha(true)
                .inputs(false)
                .picker(true)
                .build(&ui);
            }
        });
        changed
    }

    pub fn draw(&mut self, ui: &imgui::Ui) {
        let mut visible = match self.check_vanilla() {
            Ok(false) | Err(_) => {
                self.visible = Visibility::Visible;
                true
            }
            Ok(true) => self.visible.as_bool(),
        };
        if !visible {
            return;
        }
        let mut changed = self.changed;
        let pref = AppContext::pref_mut();
        imgui::Window::new(im_str!("Preferences"))
            .opened(&mut visible)
            .build(&ui, || {
                ui.text("Vanilla ROM:");
                let mut vanilla_rom = imgui::ImString::new(pref.vanilla_rom.to_string_lossy());
                if ui
                    .input_text(im_str!("##vanilla"), &mut vanilla_rom)
                    .resize_buffer(true)
                    .build()
                {
                    pref.vanilla_rom = PathBuf::from(vanilla_rom.to_str());
                    self.vanilla_hash.clear();
                    changed |= true;
                }
                ui.same_line();
                if ui.button(im_str!("Browse##vanilla")) {
                    if let Some(filename) = self.file_dialog(Some("nes")) {
                        pref.vanilla_rom = PathBuf::from(&filename);
                        self.vanilla_hash.clear();
                        changed |= true;
                    }
                }
                match self.check_vanilla() {
                    Ok(true) => ui
                        .text_colored([0.0, 1.0, 0.0, 1.0], "SHA256 checksum matches Vanilla ROM."),
                    Ok(false) => {
                        if !(pref.vanilla_rom.exists() && pref.vanilla_rom.is_file()) {
                            ui.text_colored(
                                [1.0, 0.0, 0.0, 1.0],
                                "Please provide the location of your unmodified Zelda II ROM",
                            );
                        } else {
                            ui.text_colored(
                                [1.0, 0.0, 0.0, 1.0],
                                "SHA256 checksum does not match Vanilla ROM.",
                            )
                        }
                    }
                    Err(e) => ui
                        .text_colored([1.0, 0.0, 0.0, 1.0], im_str!("Error checking ROM: {:?}", e)),
                }

                ui.separator();
                ui.text("Emulator:");
                let mut emulator = imgui::ImString::new(&pref.emulator);
                if ui
                    .input_text(im_str!("##emulator"), &mut emulator)
                    .resize_buffer(true)
                    .build()
                {
                    pref.emulator = emulator.to_str().to_owned();
                    changed |= true;
                }
                ui.same_line();
                if ui.button(im_str!("Browse##emulator")) {
                    if let Some(filename) = self.file_dialog(None) {
                        pref.emulator = filename;
                        changed |= true;
                    }
                }

                ui.separator();
                changed |= imgui::ColorEdit::new(im_str!("Background Color"), &mut pref.background)
                    .alpha(false)
                    .inputs(false)
                    .picker(true)
                    .build(&ui);

                ui.separator();
                ui.text("MultiMap Colors:");
                changed |= self.draw_connection_colors(pref, 0, 4, ui);
                ui.same_line();
                changed |= self.draw_connection_colors(pref, 4, 8, ui);
                ui.same_line();
                changed |= self.draw_connection_colors(pref, 8, MULTIMAP_COLORS.len(), ui);

                ui.separator();
                if ui.button(im_str!("Save")) {
                    let file = AppContext::config_file("preferences.ron");
                    match pref.save(&file) {
                        Err(e) => error!("Error saving preferences: {}", e),
                        _ => {}
                    }
                    changed = false;
                    self.visible = Visibility::Hidden;
                }
                ui.same_line();
                if ui.button(im_str!("Cancel")) {
                    // FIXME: stupid.  Should maintain a local copy of preferences
                    // and discard that upon cancel.
                    let file = AppContext::config_file("preferences.ron");
                    *pref = Preferences::from_file(&file).unwrap_or_default();
                    changed = false;
                    self.visible = Visibility::Hidden;
                }
            });
        self.changed = changed;
        self.visible.change(visible, self.changed);
        if Some(true)
            == self.visible.draw(
                im_str!("Preferences Changed"),
                "There are unsaved changes in Preferences.\nDo you want to discard them?",
                ui,
            )
        {
            let file = AppContext::config_file("preferences.ron");
            *pref = Preferences::from_file(&file).unwrap_or(Preferences::new());
            self.changed = false;
        }
    }
}

#[pymethods]
impl PreferencesGui {
    #[getter]
    fn get_background(&self) -> [f32; 3] {
        let pref = AppContext::pref();
        pref.background
    }
    #[setter]
    fn set_background(&self, value: [f32; 3]) {
        AppContext::pref_mut().background = value;
    }

    #[getter]
    fn get_emulator(&self) -> String {
        let pref = AppContext::pref();
        pref.emulator.clone()
    }
    #[setter]
    fn set_emulator(&self, value: &str) {
        AppContext::pref_mut().emulator = value.to_owned();
    }
}
