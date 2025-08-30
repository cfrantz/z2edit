use anyhow::Result;
use imgui::TreeNodeFlags;
use imgui::{TableColumnFlags, TableColumnSetup, TableFlags};
use python_gui::Image;

use crate::gui::util::{edit_tree_node, TreeAction};
use crate::gui::{ErrorDialog, Gui, GuiTree, Visibility};
use crate::util::tile_cache::GfxCache;
use crate::zelda2::chr::{ChrBank, ChrSchema, Layout};
use crate::zelda2::project::Project;
use crate::zelda2::vchr::{config, VirtualChr};

impl GuiTree for config::VirtualChr {
    fn tree_node(&self, ui: &imgui::Ui, path: &str, project: &Project) -> TreeAction {
        let mut result = TreeAction::None;
        if ui.collapsing_header(format!("Virtual Banks"), TreeNodeFlags::empty()) {
            for bank in 0..self.banks {
                let item = format!("{path}/{bank}");
                result.set(edit_tree_node(
                    ui,
                    &format!("Virtual Bank {bank}"),
                    &item,
                    project,
                ));
            }
        }
        result
    }
}

pub struct VirtualChrEditor {
    window_id: usize,
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    vchr: VirtualChr,
    image: Vec<Image>,
    scale: i32,
}

impl VirtualChrEditor {
    const HEADER: [&'static str; 3] = ["Sprite Page 0", "Sprite Page 1", "Background"];
    pub fn new(vchr: &VirtualChr, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(VirtualChrEditor {
            window_id: rand::random(),
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            vchr: vchr.clone(),
            image: Vec::new(),
            scale: 4,
        }))
    }

    fn commit(&self, project: &mut Project) -> Result<()> {
        project.commit(&self.path, Box::new(self.vchr.clone()))?;
        GfxCache::clear(project);
        Ok(())
    }

    fn create_images(&mut self, project: &mut Project) -> Result<()> {
        self.image.clear();
        for bank in self.vchr.data.iter() {
            let chr = project.data_ref::<ChrBank>(&format!("/chr/{bank}"))?;
            self.image
                .push(chr.create_image(self.vchr.border as u32, self.vchr.layout)?);
        }
        Ok(())
    }

    fn editor(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        if self.image.is_empty() {
            self.create_images(project)?;
        }
        let mut changed = false;
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
            .input_scalar("Border", &mut self.vchr.border)
            .step(1)
            .build()
        {
            self.vchr.border = self.vchr.border.clamp(0, 2);
            changed |= true;
        }
        width.end();
        ui.same_line();

        let width = ui.push_item_width(200.0);
        let mut layout = self.vchr.layout as usize;
        if ui.combo_simple_string("Layout", &mut layout, &["Tiles (8x8)", "Sprites (8x16)"]) {
            self.vchr.layout = match layout {
                0 => Layout::Tile,
                _ => Layout::Sprite,
            };
            changed |= true;
        }
        width.end();

        if let Some(_table) =
            ui.begin_table_with_flags("image", 3, TableFlags::RESIZABLE | TableFlags::BORDERS)
        {
            let scale = self.scale as f32;
            ui.table_setup_column_with(TableColumnSetup {
                name: "Bank",
                flags: TableColumnFlags::WIDTH_FIXED,
                init_width_or_weight: 100.0,
                ..Default::default()
            });
            ui.table_setup_column_with(TableColumnSetup {
                name: "",
                flags: TableColumnFlags::WIDTH_FIXED,
                init_width_or_weight: 48.0,
                ..Default::default()
            });

            for (i, bank) in self.vchr.data.iter_mut().enumerate() {
                let _row_id = ui.push_id_usize(i);
                if i % 4 == 0 {
                    ui.table_next_row();
                    ui.table_next_column();
                    ui.table_header("Bank");
                    ui.table_next_column();
                    ui.table_header("");
                    ui.table_next_column();
                    ui.table_header(Self::HEADER[i / 4]);

                    ui.table_next_row();
                    ui.table_next_column();
                    ui.table_next_column();
                    ui.table_next_column();
                    let [x, y] = ui.cursor_pos();
                    for n in 0..16 {
                        let xofs = n as f32 * (self.vchr.border as f32 + 8.0) * scale;
                        ui.set_cursor_pos([x + xofs, y]);
                        let id = if layout == 0 { n } else { n * 2 };
                        ui.text(format!("{id:02x}"));
                    }
                }

                ui.table_next_row();
                ui.table_next_column();
                let width = ui.push_item_width(-1.0);
                changed |= ui
                    .input_scalar("##bank", bank)
                    .display_format("%02X")
                    .chars_hexadecimal(true)
                    .build();
                width.end();

                ui.table_next_column();
                let [x, y] = ui.cursor_pos();
                let size = match self.vchr.schema {
                    ChrSchema::Mmc1_4k => 16,
                    ChrSchema::Mmc5_1k => 4,
                };
                for n in 0..size {
                    let id = 0x40 * (i % 4) + (n << 4);
                    if layout == 0 {
                        let yofs = n as f32 * (self.vchr.border as f32 + 8.0) * scale;
                        ui.set_cursor_pos([x, y + yofs]);
                        ui.text(format!("{id:02x}"));
                    } else if n & 1 == 0 {
                        let yofs = (n / 2) as f32 * (self.vchr.border as f32 + 16.0) * scale;
                        ui.set_cursor_pos([x, y + yofs]);
                        ui.text(format!("{id:02x}"));
                    }
                }

                ui.table_next_column();
                self.image[i].draw(scale, ui);
            }
        }

        if changed {
            self.create_images(project)?;
        }
        self.changed |= changed;
        Ok(())
    }
}

impl Gui for VirtualChrEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &mut Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("VirtualChr##{}", self.window_id))
            .opened(&mut visible)
            .unsaved_document(self.changed)
            .size([1280.0, 720.0], imgui::Condition::FirstUseEver)
            .build(|| self.editor(ui, project))
            .unwrap_or(Ok(()));
        self.error.draw(ui);
        self.visible.change(visible, self.changed);
        self.visible.draw(
            "Virtual CHR Changed",
            "There are unsaved chagnes in the Virtual CHR Editor.\nDo you want to discard them?",
            ui,
        );
        result
    }
    fn wants_dispose(&self) -> bool {
        self.visible == Visibility::Dispose
    }
}
