use imgui;
use imgui::im_str;
use imgui::ImStr;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Selector {
    selected: usize,
    next: usize,
    open: bool,
}

impl Selector {
    pub fn new(value: usize) -> Self {
        Selector {
            selected: value,
            next: value,
            open: false,
        }
    }

    pub fn as_mut(&mut self) -> &mut usize {
        &mut self.next
    }

    pub fn set(&mut self, value: usize) {
        self.selected = value;
        self.next = value;
    }

    pub fn value(&self) -> usize {
        self.selected
    }

    pub fn draw(&mut self, changed: bool, id: &ImStr, text: &str, ui: &imgui::Ui) -> bool {
        let mut use_next = false;
        if self.selected != self.next {
            if changed {
                if !self.open {
                    ui.open_popup(id);
                    self.open = true;
                }
                imgui::PopupModal::new(id).build(ui, || {
                    ui.text(text);
                    ui.text("\n");
                    if ui.button(im_str!(" Yes ")) {
                        self.selected = self.next;
                        use_next = true;
                        self.open = false;
                        ui.close_current_popup();
                    }
                    ui.same_line();
                    if ui.button(im_str!("  No  ")) {
                        self.next = self.selected;
                        self.open = false;
                        ui.close_current_popup();
                    }
                });
            } else {
                self.selected = self.next;
                use_next = true;
            }
        }
        use_next
    }
}
