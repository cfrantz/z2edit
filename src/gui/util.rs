use imgui;
use imgui::Key;
use std::mem::swap;

pub enum KeyAction {
    None,
    Cut,
    Copy,
    Paste,
    SelectAll,
    Undo,
    Redo,
}

impl KeyAction {
    pub fn get(ui: &imgui::Ui) -> KeyAction {
        let io = ui.io();

        if io.key_ctrl {
            if ui.is_key_pressed(ui.key_index(Key::X)) {
                KeyAction::Cut
            } else if ui.is_key_pressed(ui.key_index(Key::C)) {
                KeyAction::Copy
            } else if ui.is_key_pressed(ui.key_index(Key::V)) {
                KeyAction::Paste
            } else if ui.is_key_pressed(ui.key_index(Key::A)) {
                KeyAction::SelectAll
            } else if ui.is_key_pressed(ui.key_index(Key::Z)) {
                KeyAction::Undo
            } else if ui.is_key_pressed(ui.key_index(Key::Y)) {
                KeyAction::Redo
            } else {
                KeyAction::None
            }
        } else {
            KeyAction::None
        }
    }
}

pub fn text_outlined(ui: &imgui::Ui, color: [f32; 4], text: &imgui::ImStr) {
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    let pos = ui.cursor_pos();
    ui.set_cursor_pos([pos[0] - 1.0, pos[1] - 1.0]);
    ui.text_colored(BLACK, text);
    ui.set_cursor_pos([pos[0] + 0.0, pos[1] - 1.0]);
    ui.text_colored(BLACK, text);
    ui.set_cursor_pos([pos[0] + 1.0, pos[1] - 1.0]);
    ui.text_colored(BLACK, text);
    ui.set_cursor_pos([pos[0] - 1.0, pos[1] + 0.0]);
    ui.text_colored(BLACK, text);
    ui.set_cursor_pos([pos[0] + 1.0, pos[1] + 0.0]);
    ui.text_colored(BLACK, text);
    ui.set_cursor_pos([pos[0] - 1.0, pos[1] + 1.0]);
    ui.text_colored(BLACK, text);
    ui.set_cursor_pos([pos[0] + 0.0, pos[1] + 1.0]);
    ui.text_colored(BLACK, text);
    ui.set_cursor_pos([pos[0] + 1.0, pos[1] + 1.0]);
    ui.text_colored(BLACK, text);
    ui.set_cursor_pos([pos[0] + 0.0, pos[1] + 0.0]);
    ui.text_colored(color, text);
}

pub fn tooltip(text: &str, ui: &imgui::Ui) {
    if ui.is_item_hovered() {
        ui.tooltip_text(text);
    }
}

#[derive(Debug, Default)]
pub struct DragHelper {
    number: Option<usize>,
    amount: [f32; 2],
}

impl DragHelper {
    pub fn start(&mut self, number: usize) {
        if self.number.is_none() {
            self.number = Some(number);
            self.amount = [0.0, 0.0];
        }
    }

    pub fn position(&mut self, number: usize, position: [f32; 2]) {
        if Some(number) == self.number {
            self.amount[0] = position[0];
            self.amount[1] = position[1];
        }
    }

    pub fn drag(&mut self, number: usize, amount: [f32; 2]) {
        if Some(number) == self.number {
            self.amount[0] += amount[0];
            self.amount[1] += amount[1];
        }
    }

    pub fn delta(&self, number: usize) -> [f32; 2] {
        if Some(number) == self.number {
            self.amount
        } else {
            [0.0, 0.0]
        }
    }

    pub fn finalize(&mut self, number: usize) -> Option<[f32; 2]> {
        if Some(number) == self.number {
            self.number = None;
            Some(self.amount)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SelectBox {
    pub x0: isize,
    pub y0: isize,
    pub x1: isize,
    pub y1: isize,
}

impl Default for SelectBox {
    fn default() -> Self {
        SelectBox {
            x0: isize::MIN,
            y0: isize::MIN,
            x1: isize::MIN,
            y1: isize::MIN,
        }
    }
}

impl SelectBox {
    pub fn contains(&self, x: isize, y: isize) -> bool {
        x >= self.x0 && x <= self.x1 && y >= self.y0 && y <= self.y1
    }

    pub fn init(&mut self, x: isize, y: isize) {
        self.x0 = x;
        self.y0 = y;
        self.x1 = isize::MIN;
        self.y1 = isize::MIN;
    }

    pub fn drag(&mut self, x: isize, y: isize) {
        self.x1 = x;
        self.y1 = y;
        if self.x0 == isize::MIN {
            self.x0 = self.x1
        }
        if self.y0 == isize::MIN {
            self.y0 = self.y1
        }
    }

    pub fn normalized(self) -> Self {
        let mut norm = self;
        if norm.x1 < norm.x0 {
            swap(&mut norm.x0, &mut norm.x1);
        }
        if norm.y1 < norm.y0 {
            swap(&mut norm.y0, &mut norm.y1);
        }
        norm
    }

    pub fn valid(&self) -> bool {
        self.x0 != isize::MIN
            && self.y0 != isize::MIN
            && self.x1 != isize::MIN
            && self.y1 != isize::MIN
    }
}
