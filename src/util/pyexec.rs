use pyo3::prelude::*;
use pyo3::types::PyModule;

use crate::errors::*;
use crate::gui::console::{Console, Executor};

pub struct PythonExecutor {
    interp: PyObject,
    source: String,
    more: bool,
}

impl PythonExecutor {
    pub fn new(py: Python) -> Result<Self> {
        let module = PyModule::from_code(
            py,
            include_str!("../../python/console.py"),
            "console.py",
            "console",
        )?;

        let interp = module.call0("CreatePythonConsole")?;
        Ok(PythonExecutor {
            interp: interp.extract()?,
            source: "".to_owned(),
            more: false,
        })
    }
}

impl Executor for PythonExecutor {
    fn exec(&mut self, line: &str, console: &Console) {
        if self.more {
            self.source.push('\n');
            self.source.push_str(line);
        } else {
            self.source = line.to_owned();
        }
        Python::with_gil(|py| {
            let result = self
                .interp
                .call_method(py, "runsource", (&self.source, "<input>"), None);
            match result {
                Ok(more) => {
                    self.more = more.extract(py).unwrap();

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
                }
                Err(e) => {
                    error!("PythonConsole error {:?}", e);
                }
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
