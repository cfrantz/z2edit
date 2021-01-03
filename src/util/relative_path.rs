use serde::{Deserialize, Serialize};
use std::convert::{From, Into};

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

    pub fn relative_path(&self, other: &Path) -> Result<Option<PathBuf>> {
        let empty = PathBuf::new();
        let buf = self.buf.borrow();
        if *buf == empty {
            return Err(ErrorKind::NotFound(format!(
                "Cannot create a relative path to an empty path"
            ))
            .into());
        }
        let other = other.canonicalize()?;
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

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "String")]
#[serde(into = "String")]
pub struct PathConverter(PathBuf);

impl PathConverter {
    pub fn as_ref(&self) -> &Path {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_string_lossy().to_string()
    }
}

#[cfg(target_os = "windows")]
impl From<&str> for PathConverter {
    fn from(a: &str) -> Self {
        let b = a.replace("/", "\\");
        info!("PathConverter: {} => {}", a, b);
        PathConverter(PathBuf::from(b))
    }
}

#[cfg(not(target_os = "windows"))]
impl From<&str> for PathConverter {
    fn from(a: &str) -> Self {
        PathConverter(PathBuf::from(a.replace("\\", "/")))
    }
}

impl From<String> for PathConverter {
    fn from(a: String) -> Self {
        PathConverter::from(a.as_str())
    }
}

impl From<&String> for PathConverter {
    fn from(a: &String) -> Self {
        PathConverter::from(a.as_str())
    }
}

impl From<PathConverter> for String {
    fn from(a: PathConverter) -> Self {
        a.to_string()
    }
}

impl From<PathBuf> for PathConverter {
    fn from(a: PathBuf) -> Self {
        PathConverter(a)
    }
}

impl From<&PathBuf> for PathConverter {
    fn from(a: &PathBuf) -> Self {
        PathConverter(a.clone())
    }
}

impl From<PathConverter> for PathBuf {
    fn from(a: PathConverter) -> Self {
        a.0
    }
}

impl From<&PathConverter> for PathBuf {
    fn from(a: &PathConverter) -> Self {
        a.0.clone()
    }
}
