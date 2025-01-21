use crate::gui::{ErrorDialog, Gui, GuiTree, Visibility};
use crate::zelda2::chr::{config, ChrBank, Layout};
use crate::zelda2::project::Project;
use anyhow::Result;

use imgui::TreeNodeFlags;
use python_gui::Image;

impl GuiTree for config::ChrMemory {
    fn tree_node(&self, ui: &imgui::Ui, path: &str) -> Option<String> {
        let mut result = None;
        if ui.collapsing_header(format!("CHR Banks"), TreeNodeFlags::empty()) {
            for bank in 0..self.banks {
                let item = format!("{path}/{bank}");
                ui.tree_node_config(format!("CHR Bank {bank}##{item}"))
                    .leaf(true)
                    .build(|| {});
                if let Some(_token) = ui.begin_popup_context_item() {
                    if ui.menu_item("Edit") {
                        result = Some(item);
                    }
                }
            }
        }
        result
    }
}

pub struct ChrBankEditor {
    visible: Visibility,
    error: ErrorDialog,
    changed: bool,
    path: String,
    chr: ChrBank,
    image: Image,
    scale: i32,
}

impl ChrBankEditor {
    pub fn new(chr: &ChrBank, path: &str) -> Result<Box<dyn Gui>> {
        Ok(Box::new(ChrBankEditor {
            visible: Visibility::Visible,
            error: ErrorDialog::default(),
            changed: false,
            path: path.into(),
            chr: chr.clone(),
            image: chr.create_image(chr.border as u32, chr.layout)?,
            scale: 4,
        }))
    }

    fn editor(&mut self, ui: &imgui::Ui, _project: &Project) -> Result<()> {
        //let cfg = project.config.get::<config::PaletteGroup>(&self.path)?;
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
        ui.same_line();
        width.end();

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

        self.image.draw(self.scale as f32, ui);
        Ok(())
    }
}

impl Gui for ChrBankEditor {
    fn draw(&mut self, ui: &imgui::Ui, project: &Project) -> Result<()> {
        let mut visible = self.visible.as_bool();
        if !visible {
            return Ok(());
        }
        let result = ui
            .window(format!("ChrBank##{}", self.path))
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
    fn window_id(&self) -> u64 {
        0
    }
}
