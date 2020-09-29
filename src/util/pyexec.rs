use crate::gui::console::{Console, Executor};
use pyo3::prelude::Python;
use pyo3::types::{PyAny, PyModule};

pub struct PythonExecutor<'p> {
    module: &'p PyModule,
    interp: &'p PyAny,
    source: String,
    more: bool,
}

impl<'p> PythonExecutor<'p> {
    pub fn new(py: Python<'p>) -> Self {
        let m = PyModule::from_code(py,
                            include_str!("../../python/console.py"),
                            "console.py",
                            "console").unwrap();

        let interp = m.call0("CreatePythonConsole").unwrap();
        PythonExecutor {
            module: m,
            interp: interp,
            source: "".to_owned(),
            more: false,
        }
    }
}

impl Executor for PythonExecutor<'_> {
    fn exec(&mut self, line: &str, console: &Console) {
        if self.more {
            self.source.push('\n');
            self.source.push_str(line);
        } else {
            self.source = line.to_owned();
        }
        let result = self.interp.call_method("runsource", (&self.source, "<input>"), None).unwrap();
        self.more = result.extract().unwrap();

        let s = self.interp.call_method0("GetOut").unwrap().extract::<String>().unwrap();
        if !s.is_empty() {
            console.add_item(0x33ff33, &s);
        }

        let s = self.interp.call_method0("GetErr").unwrap().extract::<String>().unwrap();
        if !s.is_empty() {
            console.add_item(0x3333ff, &s);
        }
    }

    fn prompt(&self) -> &str {
        if self.more {
            "..."
        } else {
            ">>>"
        }
    }
}
