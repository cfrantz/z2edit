use imgui;
use imgui::im_str;
use imgui::ImString;
use std::cell::{Cell, RefCell};
use std::sync::mpsc;
use std::thread;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::gui::glhelper;

struct Item {
    color: [f32; 4],
    text: String,
}

pub struct Console {
    pub visible: bool,
    name: String,
    input: ImString,
    items: RefCell<Vec<Item>>,
    scroll: Cell<bool>,
    queue: mpsc::Receiver<String>,
}

pub trait Executor {
    fn exec(&mut self, line: &str, console: &Console);
}

pub struct NullExecutor;

impl Executor for NullExecutor {
    fn exec(&mut self, line: &str, console: &Console) {
        console.add_text(line);
    }
}

impl Console {
    pub fn new(name: &str) -> Self {
        let (sender, receiver) = mpsc::sync_channel(2);
        let _ = thread::spawn(move || {
            Console::stdin_listener(sender);
        });
        Console {
            visible: false,
            name: name.to_owned(),
            input: ImString::with_capacity(256),
            items: RefCell::new(Vec::<Item>::new()),
            scroll: Cell::new(true),
            queue: receiver,
        }
    }

    fn stdin_listener(sender: mpsc::SyncSender<String>) {
        let mut rl = Editor::<()>::new();
        let ret = loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => sender.send(line).unwrap(),
                Err(ReadlineError::Interrupted) => {
                    break 1;
                },
                Err(ReadlineError::Eof) => {
                    break 0;
                },
                Err(e) => { 
                    error!("Readline: {}", e);
                    break 255;
                },
            }
        };
        std::process::exit(ret);
    }

    pub fn clear(&self) {
        self.items.borrow_mut().clear();
    }

    pub fn add_item(&self, color: u32, text: &str) {
        let item = Item {
            color: glhelper::color_as_f32(color | 0xFF000000),
            text: text.to_owned(),
        };
        self.items.borrow_mut().push(item);
        self.scroll.set(true);
    }

    pub fn add_text(&self, text: &str) {
        let (color, text) = if text.starts_with("#{") && text.find('}') == Some(5) {
            let c = u32::from_str_radix(&text[2..5], 16).unwrap_or(0xF00);
            let r = ((c & 0xF00) >> 8) * 0x11;
            let g = ((c & 0x0F0) >> 4) * 0x11;
            let b = ((c & 0x00F) >> 0) * 0x11;
            (r | (g<<8) | (b<<16), &text[6..])
        } else if text.starts_with('#') {
            (0x88ccff, text)
        } else {
            (0xffffff, text)
        };
        self.add_item(color, text);
    }

    pub fn draw(&mut self, exec: &mut dyn Executor, ui: &imgui::Ui) {
        match self.queue.try_recv() {
            Ok(line) => exec.exec(&line, self),
            Err(_) => {},
        };
        if !self.visible {
            return;
        }
        let mut visible = self.visible;
        let name = ImString::new(&self.name);
        imgui::Window::new(&name)
            .opened(&mut visible)
            .scroll_bar(false)
            .build(ui, || {
                if ui.small_button(im_str!("Clear")) {
                    self.clear();
                }
                imgui::ChildWindow::new(imgui::Id::Int(1))
                    .size([0.0, -ui.text_line_height_with_spacing()])
                    .border(true)
                    .build(ui, || {
                        let spacing = ui.push_style_var(imgui::StyleVar::ItemSpacing([4.0, 1.0]));
                        for item in self.items.borrow().iter() {
                            ui.text_colored(item.color, &item.text);
                        }
                        if self.scroll.get() {
                            ui.set_scroll_here_y();
                        }
                        self.scroll.set(false);
                        spacing.pop(ui);
                });
                ui.text(">>>");
                ui.same_line(0.0);
                if imgui::InputText::new(ui, im_str!(""), &mut self.input)
                    .enter_returns_true(true)
                    .build() {
                    exec.exec(self.input.to_str(), self);
                    self.input.clear();
                    ui.set_keyboard_focus_here(imgui::FocusedWidget::Previous);
                }
                if ui.is_item_hovered() {
                    ui.set_keyboard_focus_here(imgui::FocusedWidget::Previous);
                }

            });
        self.visible = visible;
    }
}
