use imgui;
use imgui::Key;

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
