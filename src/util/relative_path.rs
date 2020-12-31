use pathdiff::diff_paths;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::errors::*;

#[derive(Debug, Default, Clone)]
pub struct RelativePath {
    pub buf: Rc<RefCell<PathBuf>>,
}

impl RelativePath {
    pub fn set(&self, path: &Path) {
        self.buf.replace(path.to_path_buf());
    }

    pub fn relative_path<P>(&self, other: P) -> Result<Option<PathBuf>>
    where
        P: AsRef<Path>,
    {
        let empty = PathBuf::new();
        let buf = self.buf.borrow();
        if *buf == empty {
            return Err(ErrorKind::NotFound(format!(
                "Cannot create a relative path to an empty path"
            ))
            .into());
        }
        Ok(diff_paths(other, &*buf))
    }

    pub fn path<P>(&self, path: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        let buf = self.buf.borrow();
        buf.join(path)
    }

    pub fn subdir(&self) -> PathBuf {
        self.buf.borrow().clone()
    }
}
