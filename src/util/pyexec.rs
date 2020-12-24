use pyo3::prelude::*;
use pyo3::types::PyModule;

use crate::errors::*;
use crate::gui::console::{Console, Executor};

pub struct PythonExecutor {
    interp: PyObject,
    more: bool,
}

impl PythonExecutor {
    pub fn new(py: Python) -> Result<Self> {
        let module = PyModule::import(py, "z2edit.console")?;
        let interp = module.call0("CreatePythonConsole")?;
        Ok(PythonExecutor {
            interp: interp.extract()?,
            more: false,
        })
    }
}

impl Executor for PythonExecutor {
    fn exec(&mut self, line: &str, console: &Console) {
        Python::with_gil(|py| {
            if line == "\x03" {
                let _ = self.interp.call_method(py, "resetbuffer", (), None);
                self.more = false;
            } else {
                let result = self.interp.call_method(py, "push", (line,), None);
                match result {
                    Ok(more) => {
                        self.more = more.extract(py).unwrap();
                    }
                    Err(e) => {
                        error!("PythonConsole error {:?}", e);
                    }
                }
            }
        });
        self.process_output(console);
    }

    fn process_output(&self, console: &Console) {
        Python::with_gil(|py| {
            let s = self
                .interp
                .call_method0(py, "GetOut")
                .unwrap()
                .extract::<String>(py)
                .unwrap();
            if !s.is_empty() {
                console.add_item(0x33ff33, &s);
            }

            let s = self
                .interp
                .call_method0(py, "GetErr")
                .unwrap()
                .extract::<String>(py)
                .unwrap();
            if !s.is_empty() {
                console.add_item(0x3333ff, &s);
            }
        });
    }

    fn prompt(&self) -> &str {
        if self.more {
            "..."
        } else {
            ">>>"
        }
    }
}
