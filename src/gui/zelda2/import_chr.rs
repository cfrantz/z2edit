use std::borrow::Cow;
use std::convert::{From, TryFrom};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use imgui;
use imgui::im_str;
use imgui::ImString;

use crate::errors::*;
use crate::gui::glhelper::Image;
use crate::gui::zelda2::tile_cache::{Schema, TileCache};
use crate::gui::zelda2::Gui;
use crate::gui::ErrorDialog;
use crate::gui::{Selector, Visibility};
use crate::nes::Address;
use crate::util::clamp;
use crate::zelda2::config::{ChrBankScheme, Config};
use crate::zelda2::import_chr::{ChrBankKind, ImportChrBank};
use crate::zelda2::project::{Edit, Project};

pub struct ImportChrBankGui {
    visible: Visibility,
    changed: bool,
    is_new: bool,
    edit: Rc<Edit>,
    names: Vec<ImString>,
    schemes: Vec<ImString>,
    filename: ImString,
    scheme: Selector,
    selector: Selector,
    cache: TileCache,
    image: Image,
    overlay_image: Option<Image>,
    import: ImportChrBank,
    error: ErrorDialog,
}

impl ImportChrBankGui {
    pub fn new(project: &Project, edit: Option<Rc<Edit>>) -> Result<Box<dyn Gui>> {
        if let Some(edit) = &edit {
            let e = edit.edit.borrow();
            if !e.as_any().is::<ImportChrBank>() {
                return Err(ErrorKind::RomDataError(format!(
                    "Expected edit '{}' to contain an ImportChrBank edit",
                    edit.label()
                ))
                .into());
            }
        }
        let is_new = edit.is_none();
        let edit = edit.unwrap_or_else(|| project.create_edit("ImportChrBank", None).unwrap());
        let config = Config::get(&edit.config())?;

        let import = if is_new {
            ImportChrBank::default()
        } else {
            let obj = edit.edit.borrow();
            let import = obj.as_any().downcast_ref::<ImportChrBank>().unwrap();
            import.clone()
        };

        let cache = TileCache::new(&edit, import.schema());
        let image = cache.get_bank(
            Address::Chr(import.bank as isize, 0),
            &import.palette,
            import.sprite_layout,
            import.border as u32,
        )?;
        let filename = ImString::new(&import.file.to_string());

        let schemes: Vec<ImString> = match config.misc.chr_bank_scheme {
            ChrBankScheme::Vanilla => vec![ImString::new("Vanilla")],
            ChrBankScheme::MMC5_12By1K(_) => vec![
                ImString::new("Vanilla"),
                ImString::new("MMC5 1k "),
                ImString::new("MMC5 VBanks"),
            ],
        };
        let kind = import.kind as usize;

        let selected = import.bank;
        let mut obj = Box::new(ImportChrBankGui {
            visible: Visibility::Visible,
            changed: false,
            is_new: is_new,
            edit: edit,
            names: Vec::new(),
            schemes: schemes,
            filename: filename,
            cache: cache,
            image: image,
            overlay_image: None,
            scheme: Selector::new(kind),
            selector: Selector::new(selected),
            import: import,
            error: ErrorDialog::default(),
        });
        obj.make_names()?;
        Ok(obj)
    }

    fn make_names(&mut self) -> Result<()> {
        let config = Config::get(&self.edit.config())?;
        self.names.clear();
        match self.import.schema() {
            Schema::MMC1_4k => {
                for i in 0..config.layout.segment("chr")?.banks() {
                    self.names.push(im_str!("CHR Bank ${:02x}", i));
                }
            }
            Schema::MMC5_1k => {
                for i in 0..config.layout.segment("chr")?.banks() * 4 {
                    self.names.push(im_str!("CHR Bank ${:02x}", i));
                }
            }
            Schema::VBanks => {
                for i in 0..(256 / 12) {
                    self.names.push(im_str!("VBank ${:02x}", i));
                }
            }
            _ => {
                log::error!("Unimplemented: {:?}", self.import.schema());
            }
        }
        Ok(())
    }

    fn set_kind(&mut self, index: usize) {
        let x = ChrBankKind::try_from(index);
        self.import.kind = x.unwrap_or(self.import.kind);
    }

    pub fn commit(&mut self, project: &mut Project) -> Result<()> {
        if self.edit.label().is_empty() {
            self.edit
                .set_label_suffix(&format!("bank ${:02x}", self.selector.value()));
        }
        project.commit(&self.edit, Box::new(self.import.clone()))?;
        self.is_new = false;
        self.cache = TileCache::new(&self.edit, self.import.schema());
        self.refresh_image();
        self.overlay_image = None;
        Ok(())
    }

    fn refresh_image(&mut self) {
        self.image = self
            .cache
            .get_bank(
                Address::Chr(self.import.bank as isize, 0),
                &self.import.palette,
                self.import.sprite_layout,
                self.import.border as u32,
            )
            .expect("refresh_image");
        if let Some(image) = &self.overlay_image {
            info!("did overlay");
            self.image.overlay(image, 0, 0);
            self.image.update();
        }
    }

    fn file_dialog(&self, ftype: Option<&str>) -> Option<String> {
        let result = nfd::open_file_dialog(ftype, None).expect("ImportChrBankGui::file_dialog");
        match result {
            nfd::Response::Okay(path) => Some(path),
            _ => None,
        }
    }

    fn save_image(&self) {
        let result =
            nfd::open_save_dialog(Some("bmp"), None).expect("ImportChrBankGui::save_image");
        match result {
            nfd::Response::Okay(filename) => match self.image.save_bmp(&filename) {
                Ok(_) => {}
                Err(e) => self
                    .error
                    .show("Save Image", "Could not save BMP Image", Some(e)),
            },
            _ => {}
        }
    }

    fn load_image(&mut self) {
        let fileresult = self
            .edit
            .subdir
            .relative_path(Path::new(self.filename.to_str()));
        let filepath = match fileresult {
            Ok(Some(p)) => p,
            Ok(None) => PathBuf::from(self.filename.to_str()),
            Err(e) => {
                self.error.show("Load Image", "Could not determine the image filepath relative to the project.\nPlease save the project first.", Some(e));
                return;
            }
        };
        info!(
            "Computing relative path: {:?} => {:?}",
            self.filename.to_str(),
            filepath
        );
        let new_path = self.edit.subdir.path(&filepath);
        self.overlay_image = match Image::load_bmp(&new_path) {
            Ok(img) => {
                self.import.file = filepath.into();
                Some(img)
            }
            Err(e) => {
                self.error
                    .show("Load Image", "Could not load BMP Image", Some(e));
                None
            }
        };
    }

    fn draw_image(&mut self, ui: &imgui::Ui) -> bool {
        let mut changed = false;
        changed |= ui.checkbox(im_str!("Sprite Layout"), &mut self.import.sprite_layout);
        let width = ui.push_item_width(100.0);
        if ui
            .input_int(im_str!("Border"), &mut self.import.border)
            .build()
        {
            self.import.border = clamp(self.import.border, 0, 4);
            changed |= true;
        }
        width.pop(ui);

        ui.text("File:");
        ui.same_line();
        if ui
            .input_text(im_str!("##file"), &mut self.filename)
            .resize_buffer(true)
            .enter_returns_true(true)
            .build()
        {
            self.load_image();
            changed |= true;
        }
        ui.same_line();
        if ui.button(im_str!("Browse##file")) {
            if let Some(filename) = self.file_dialog(Some("bmp")) {
                self.filename = ImString::new(filename);
                self.load_image();
                changed |= true;
            }
        }

        ui.separator();

        let origin = ui.cursor_pos();
        let scale = 2.0;
        ui.text("");
        ui.text("   ");
        ui.same_line();
        self.image.draw(scale, ui);

        for x in 0..16 {
            ui.set_cursor_pos([
                origin[0] + 32.0 + scale * (x * (8 + self.import.border)) as f32,
                origin[1],
            ]);
            ui.text(im_str!(
                "{:02x}",
                if self.import.sprite_layout { x * 2 } else { x }
            ));
        }

        let (_, h, _) = self.cache.bank_size(Address::Chr(0, 0)).unwrap();
        for y in 0..(h as i32) {
            if self.import.sprite_layout {
                if y % 2 == 0 {
                    ui.set_cursor_pos([
                        origin[0],
                        origin[1] + 20.0 + scale * (y / 2 * (16 + self.import.border)) as f32,
                    ]);
                    ui.text(im_str!("{:>3x}", y * 16));
                }
            } else {
                ui.set_cursor_pos([
                    origin[0],
                    origin[1] + 20.0 + scale * (y * (8 + self.import.border)) as f32,
                ]);

                ui.text(im_str!("{:>3x}", y * 16));
            }
        }

        if changed {
            self.refresh_image();
        }
        changed
    }
}

impl Gui for ImportChrBankGui {
    fn draw(&mut self, project: &mut Project, ui: &imgui::Ui) {
        let mut visible = self.visible.as_bool();
        if !visible {
            return;
        }
        let title = ImString::new(self.edit.title());
        imgui::Window::new(&title)
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .build(ui, || {
                let width = ui.push_item_width(200.0);
                if self.is_new {
                    imgui::ComboBox::new(im_str!("Bank")).build_simple(
                        ui,
                        self.selector.as_mut(),
                        &self.names,
                        &|x| Cow::Borrowed(&x),
                    );
                } else {
                    ui.label_text(im_str!("Bank"), &self.names[self.selector.value()]);
                }

                ui.same_line();
                if self.is_new {
                    imgui::ComboBox::new(im_str!("Scheme")).build_simple(
                        ui,
                        self.scheme.as_mut(),
                        &self.schemes,
                        &|x| Cow::Borrowed(&x),
                    );
                } else {
                    ui.label_text(im_str!("Bank"), &self.names[self.selector.value()]);
                }

                width.pop(ui);
                let mut changed = false;

                ui.same_line();
                if ui.button(im_str!("Export Image")) {
                    self.save_image();
                }
                ui.same_line();
                if ui.button(im_str!("Commit")) {
                    match self.commit(project) {
                        Err(e) => self.error.show("ImportChrBankGui", "Commit Error", Some(e)),
                        _ => {}
                    };
                    self.changed = false;
                }
                ui.separator();
                changed |= self.draw_image(ui);
                self.changed |= changed;
            });
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            im_str!("CHR Import Changed"),
            "There are unsaved changes in the CHR Import Editor.\nDo you want to discard them?",
            ui,
        );
        if self.selector.draw(
            self.changed,
            im_str!("CHR Import Changed"),
            "There are unsaved changes in the CHR Import Editor.\nDo you want to discard them?",
            ui,
        ) {
            self.changed = false;
            self.import = ImportChrBank::default();
            self.import.bank = self.selector.value();
            self.refresh_image();
        }
        if self.scheme.draw(
            self.changed,
            im_str!("CHR Import Changed"),
            "There are unsaved changes in the CHR Import Editor.\nDo you want to discard them?",
            ui,
        ) {
            self.changed = false;
            self.import = ImportChrBank::default();
            self.import.bank = self.selector.value();
            self.set_kind(self.scheme.value());
            self.make_names().unwrap();
            self.selector.set(0);
            self.cache.reset(self.import.schema());
            self.refresh_image();
        }
    }

    fn refresh(&mut self) {
        self.refresh_image();
    }

    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
    fn window_id(&self) -> u64 {
        self.edit.random_id
    }
}
