use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;

use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::errors::*;
use crate::gui::app_context::AppContext;
use crate::gui::zelda2::python::PythonScriptGui;
use crate::gui::zelda2::Gui;
use crate::zelda2::project::{Edit, EditProxy, Project, RomData};

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct PythonScript {
    pub file: Option<String>,
    pub code: String,
}

impl PythonScript {
    pub fn create(id: Option<&str>) -> Result<Box<dyn RomData>> {
        if id.is_none() {
            Ok(Box::new(Self::default()))
        } else {
            Err(ErrorKind::IdPathError("id forbidden".to_string()).into())
        }
    }
}

#[typetag::serde]
impl RomData for PythonScript {
    fn name(&self) -> String {
        "Python Script".to_owned()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn unpack(&mut self, _edit: &Rc<Edit>) -> Result<()> {
        Ok(())
    }

    fn pack(&self, edit: &Rc<Edit>) -> Result<()> {
        let contents;
        let code = if let Some(file) = &self.file {
            let filepath = edit.subdir.path(file);
            contents = std::fs::read_to_string(&filepath)?;
            &contents
        } else {
            &self.code
        };
        let gil = Python::acquire_gil();
        let py = gil.python();
        let proxy = Py::new(py, EditProxy::new(Rc::clone(edit)))?;
        let locals = PyDict::new(py);
        locals.set_item("edit", proxy)?;
        py.run(
            "from z2edit.assembler import Asm\n\
                asm = Asm(edit)\n",
            None,
            Some(locals),
        )?;
        py.run(&code, None, Some(locals))?;
        AppContext::app().borrow(py).process_python_output();
        Ok(())
    }

    fn gui(&self, project: &Project, commit_index: isize) -> Result<Box<dyn Gui>> {
        PythonScriptGui::new(project, commit_index)
    }

    fn to_text(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| e.into())
    }

    fn from_text(&mut self, text: &str) -> Result<()> {
        match serde_json::from_str(text) {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
