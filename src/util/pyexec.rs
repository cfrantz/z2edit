use crate::gui::console::{Console, Executor};
use pyo3::prelude::*;
use pyo3::types::PyModule;

#[pyclass]
pub struct PythonExecutor {
    interp: PyObject,
    source: String,
    more: bool,
}

impl PythonExecutor {
    pub fn new(py: Python, submodule: &PyModule) -> Self {
        let m = PyModule::from_code(py,
                            include_str!("../../python/console.py"),
                            "console.py",
                            "console").unwrap();

        m.add_submodule(submodule);
        let interp = m.call0("CreatePythonConsole").unwrap();

        PythonExecutor {
            interp: interp.extract().unwrap(),
            source: "".to_owned(),
            more: false,
        }

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
        let result = self.interp.call_method(py, "runsource", (&self.source, "<input>"), None).unwrap();
        self.more = result.extract(py).unwrap();

        let s = self.interp.call_method0(py, "GetOut").unwrap().extract::<String>(py).unwrap();
        if !s.is_empty() {
            console.add_item(0x33ff33, &s);
        }

        let s = self.interp.call_method0(py, "GetErr").unwrap().extract::<String>(py).unwrap();
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
