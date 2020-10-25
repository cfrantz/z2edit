use std::convert::{From, Into};
use serde::{Serialize, Deserialize};
use crate::errors::*;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from="String")]
#[serde(into="String")]
pub struct IdPath(pub Vec<String>);

impl IdPath {
    pub fn check_len(&self, category: &str, len: usize) -> Result<()> {
        if len == self.0.len() {
            Ok(())
        } else {
            Err(ErrorKind::IdPathBadLength(category.to_owned(), len).into())
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn at(&self, i: usize) -> &str {
        &self.0[i]
    }
}

impl From<String> for IdPath {
    fn from(a: String) -> Self {
        IdPath(a.split('/').map(|s| s.to_owned()).collect())
    }
}

impl From<&IdPath> for String {
    fn from(a: &IdPath) -> Self {
        a.0.join("/")
    }
}

impl From<IdPath> for String {
    fn from(a: IdPath) -> Self {
        a.0.join("/")
    }
}
