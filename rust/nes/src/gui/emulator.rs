use pyo3::prelude::*;
use python_gui::UiContext;

use crate::system::Nes;

#[pyclass]
pub struct EmulatorGui {
    #[pyo3(get)]
    nes: Py<Nes>,
    window_id: u32,
    #[pyo3(get)]
    wants_dispose: bool,
}

impl EmulatorGui {
    fn draw<'p>(&mut self, py: Python<'p>, ui: &imgui::Ui) {
        ui.window(format!("Emulator {}", self.window_id))
        .build(|| {
            Nes::emulate_frame(self.nes.bind(py));
            let nes = self.nes.borrow(py);
            let image = nes.image.borrow(py);
            image.draw(4.0, ui);
        });
    }
}

#[pymethods]
impl EmulatorGui {
    #[new]
    pub fn new(nes: Py<Nes>) -> Self {
        Self {
            nes,
            window_id: rand::random(),
            wants_dispose: false,
        }
    }

    #[pyo3(name = "draw")]
    fn _draw<'p>(&mut self, py: Python<'p>, ctx: &UiContext) {
        self.draw(py, ctx.ui)
    }
}