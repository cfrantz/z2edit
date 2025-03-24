use anyhow::Error;
use std::sync::Mutex;

#[derive(Default)]
pub struct ErrorDialog {
    inner: Mutex<Inner>,
}

#[derive(Default)]
struct Inner {
    id: u32,
    open: bool,
    title: String,
    message: String,
    error: Option<Error>,
    choices: Vec<String>,
    choice: Option<usize>,
}

impl ErrorDialog {
    pub fn show(&self, title: &str, message: &str, error: Error) {
        log::error!("{title}: {message} {error:?}");
        let mut inner = self.inner.lock().unwrap();
        inner.id = rand::random();
        inner.title = title.into();
        inner.message = message.into();
        inner.error = Some(error);
        inner.choices = vec!["Dismiss".to_string()];
        inner.choice = None;
    }

    pub fn show_choice(&self, title: &str, message: &str, choice: &[&str]) -> Option<usize> {
        let mut inner = self.inner.lock().unwrap();
        if inner.choice.is_some() {
            return inner.choice.take();
        }
        if inner.title.is_empty() {
            log::error!("{title}: {message}");
            inner.id = rand::random();
            inner.title = title.into();
            inner.message = message.into();
            inner.error = None;
            inner.choices = choice.iter().map(|s| s.to_string()).collect();
        }
        None
    }

    pub fn draw(&self, ui: &imgui::Ui) {
        let mut inner = self.inner.lock().unwrap();
        if !inner.title.is_empty() {
            if !inner.open {
                ui.open_popup(format!("Error: {}##{}", inner.title, inner.id));
                inner.open = true;
            }
            ui.modal_popup_config(format!("Error: {}##{}", inner.title, inner.id))
                .title_bar(true)
                .build(|| {
                    ui.text(format!("{}\n\n", inner.message));
                    if let Some(error) = &inner.error {
                        ui.text(format!("{error:?}"));
                    }
                    ui.separator();

                    for i in 0..inner.choices.len() {
                        if ui.button(&inner.choices[i]) {
                            inner.choice = Some(i);
                            inner.open = false;
                            inner.title.clear();
                            inner.message.clear();
                            ui.close_current_popup();
                        }
                        ui.same_line();
                    }
                });
        }
    }
}
