use anyhow::Result;
use hex_color::HexColor;
use imgui::TreeNodeFlags;
use imgui::{TableColumnFlags, TableColumnSetup, TableFlags};
use python_gui::fa;
use python_gui::Image;
use rfd::FileDialog;

use crate::gui::util::edit_tree_node;
use crate::gui::util::tooltip;
use crate::gui::util::TreeAction;
use crate::gui::{ErrorDialog, Gui, GuiTree, Visibility};
use crate::util::tile_cache::GfxCache;
use crate::zelda2::chr::{config, ChrBank, ChrSchema, Layout, Overlay};
use crate::zelda2::project::Project;

impl GuiTree for config::ChrMemory {
    fn tree_node(&self, ui: &imgui::Ui, path: &str, project: &Project) -> TreeAction {
        let mut result = TreeAction::None;
        if ui.collapsing_header(format!("CHR Banks"), TreeNodeFlags::empty()) {
            for bank in 0..self.banks {
                let item = format!("{path}/{bank}");
                result.set(edit_tree_node(
                    ui,
                    &format!("CHR Bank {bank} (${bank:02X})"),
                    &item,
                    project,
                ));
            }
        }
        result
    }
}

pub struct ChrBankEditor {
    window_id: usize,
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    chr: ChrBank,
    image: Image,
    old_image: Image,
    scale: i32,
}

impl ChrBankEditor {
    pub fn new(chr: &ChrBank, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(ChrBankEditor {
            window_id: rand::random(),
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            chr: chr.clone(),
            image: chr.create_image(chr.border as u32, chr.layout)?,
            old_image: chr.create_image(chr.border as u32, chr.layout)?,
            scale: 4,
        }))
    }

    fn commit(&self, project: &mut Project) -> Result<()> {
        project.commit(&self.path, Box::new(self.chr.clone()))?;
        GfxCache::clear(project);
        Ok(())
    }

    fn edit_palette_list(ui: &imgui::Ui, overlay: &mut Overlay) -> bool {
        let mut changed = false;
        let mut delindex = None;
        for (n, (color, index)) in overlay.palette.iter_mut().enumerate() {
            let _id = ui.push_id_usize(n);
            let mut col = [
                color.r as f32 / 255.0,
                color.g as f32 / 255.0,
                color.b as f32 / 255.0,
            ];
            if ui
                .color_edit3_config("##col", &mut col)
                .picker(true)
                .inputs(false)
                .build()
            {
                color.r = (col[0] * 255.0) as u8;
                color.g = (col[1] * 255.0) as u8;
                color.b = (col[2] * 255.0) as u8;
                changed |= true;
            }
            tooltip("Input Image Color", ui);
            ui.same_line();
            if ui.input_scalar("##index", index).step(1).build() {
                *index = (*index).clamp(0, 3);
                changed |= true;
            }
            tooltip("NES palette index", ui);
            ui.same_line();
            if ui.button(&format!("{}", fa::ICON_TRASH)) {
                delindex = Some(n);
                changed |= true;
            }
            tooltip("Delete palette", ui);
        }
        if let Some(i) = delindex {
            overlay.palette.remove(i);
        }
        if ui.button(&format!("{}", fa::ICON_COPY)) {
            overlay.palette.push((HexColor::rgb(0, 0, 0), 0));
            changed |= true;
        }
        tooltip("Add palette", ui);
        changed
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if ui.button("Commit") {
            match self.commit(project) {
                Ok(()) => self.changed = false,
                Err(e) => self.error.show(
                    "Commit Error",
                    &format!("Error comitting {:?}", self.path),
                    e,
                ),
            }
        }
        ui.same_line();
        ui.text(&self.path);
        let width = ui.push_item_width(100.0);
        if ui.input_scalar("Scale", &mut self.scale).step(1).build() {
            self.scale = self.scale.clamp(1, 8);
        }
        ui.same_line();
        if ui
            .input_scalar("Border", &mut self.chr.border)
            .step(1)
            .build()
        {
            self.chr.border = self.chr.border.clamp(0, 2);
            self.image = self
                .chr
                .create_image(self.chr.border as u32, self.chr.layout)?;
        }
        width.end();
        ui.same_line();

        let width = ui.push_item_width(200.0);
        let mut layout = self.chr.layout as usize;
        if ui.combo_simple_string("Layout", &mut layout, &["Tiles (8x8)", "Sprites (8x16)"]) {
            self.chr.layout = match layout {
                0 => Layout::Tile,
                _ => Layout::Sprite,
            };
            self.image = self
                .chr
                .create_image(self.chr.border as u32, self.chr.layout)?;
        }
        width.end();
        ui.same_line();

        if ui.button("Export Image") {
            if let Some(bmp) = FileDialog::new()
                .set_title("Save image")
                .add_filter("BMP", &["bmp"])
                .add_filter("All", &["*"])
                .save_file()
            {
                match self.image.save_bmp(&bmp) {
                    Ok(_) => {}
                    Err(e) => self
                        .error
                        .show("Save Error", &format!("Error saving {bmp:?}"), e),
                }
            }
        }

        if let Some(_table) =
            ui.begin_table_with_flags("image", 2, TableFlags::RESIZABLE | TableFlags::BORDERS)
        {
            let scale = self.scale as f32;
            ui.table_setup_column_with(TableColumnSetup {
                name: "",
                flags: TableColumnFlags::WIDTH_FIXED,
                init_width_or_weight: 48.0,
                ..Default::default()
            });
            ui.table_next_row();
            ui.table_next_column();
            // Empty cell

            ui.table_next_column();
            let [x, y] = ui.cursor_pos();
            for n in 0..16 {
                let xofs = n as f32 * (self.chr.border as f32 + 8.0) * scale;
                ui.set_cursor_pos([x + xofs, y]);
                let id = if layout == 0 { n } else { n * 2 };
                ui.text(format!("{id:02x}"));
            }

            ui.table_next_row();
            ui.table_next_column();
            let [x, y] = ui.cursor_pos();
            let size = match self.chr.schema {
                ChrSchema::Mmc1_4k => 16,
                ChrSchema::Mmc5_1k => 4,
            };
            for n in 0..size {
                if layout == 0 {
                    let yofs = n as f32 * (self.chr.border as f32 + 8.0) * scale;
                    ui.set_cursor_pos([x, y + yofs]);
                    ui.text(format!("{:02x}", n << 4));
                } else if n & 1 == 0 {
                    let yofs = (n / 2) as f32 * (self.chr.border as f32 + 16.0) * scale;
                    ui.set_cursor_pos([x, y + yofs]);
                    ui.text(format!("{:02x}", n << 4));
                }
            }

            ui.table_next_column();
            self.image.draw(scale, ui);
        }

        let mut changed = false;
        if let Some(_table) = ui.begin_table_header_with_flags(
            "overlay",
            [
                TableColumnSetup {
                    name: "Image File",
                    flags: TableColumnFlags::WIDTH_STRETCH,
                    ..Default::default()
                },
                TableColumnSetup {
                    name: "Image Palette",
                    flags: TableColumnFlags::WIDTH_FIXED,
                    init_width_or_weight: 300.0,
                    ..Default::default()
                },
                TableColumnSetup {
                    name: "Browse",
                    flags: TableColumnFlags::WIDTH_FIXED,
                    init_width_or_weight: 100.0,
                    ..Default::default()
                },
                TableColumnSetup {
                    name: "Del",
                    flags: TableColumnFlags::WIDTH_FIXED,
                    init_width_or_weight: 32.0,
                    ..Default::default()
                },
            ],
            TableFlags::BORDERS,
        ) {
            let mut delindex = None;
            for (i, overlay) in self.chr.overlay.iter_mut().enumerate() {
                let _id = ui.push_id_usize(i);
                ui.table_next_row();
                ui.table_next_column();
                let width = ui.push_item_width(-1.0);
                changed |= ui
                    .input_text("##filename", &mut overlay.path)
                    .enter_returns_true(true)
                    .build();
                width.end();

                ui.table_next_column();
                if !overlay.path.is_empty() {
                    changed |= Self::edit_palette_list(ui, &mut *overlay);
                }

                ui.table_next_column();
                let width = ui.push_item_width(-1.0);
                if ui.button("Browse") {
                    if let Some(bmp) = FileDialog::new()
                        .set_title("Load image")
                        .add_filter("BMP", &["bmp"])
                        .add_filter("All", &["*"])
                        .pick_file()
                    {
                        overlay.path = bmp.to_string_lossy().into();
                        changed |= true;
                    }
                }
                width.end();

                ui.table_next_column();
                if ui.button(&format!("{}", fa::ICON_TRASH)) {
                    delindex = Some(i);
                    changed |= true;
                }
                tooltip("Remove overlay", ui);
            }

            if let Some(i) = delindex {
                self.chr.overlay.remove(i);
            }
            if ui.button(&format!("{}", fa::ICON_COPY)) {
                self.chr.overlay.push(Default::default());
            }
            tooltip("Add overlay", ui);
        }

        if changed {
            match self.chr.apply() {
                Ok(_) => {}
                Err(e) => self.error.show("Load Image", "Error loading image", e),
            }
            // To prevent glitching during updates, we "double buffer" the updated
            // image.  This prevents the `Image` drop handler from destroying the
            // opengl image ID while we're still displaying it.
            self.old_image = self
                .chr
                .create_image(self.chr.border as u32, self.chr.layout)?;
            std::mem::swap(&mut self.image, &mut self.old_image);
        }
        self.changed |= changed;

        Ok(())
    }
}

impl Gui for ChrBankEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("ChrBank##{}", self.window_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Palette Changed",
            "There are unsaved chagnes in the Palette Editor.\nDo you want to discard them?",
            ui,
        );
        result
    }
    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
}
