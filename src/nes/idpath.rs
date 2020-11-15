use crate::errors::*;
use serde::{Deserialize, Serialize};
use std::convert::{From, Into};

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "String")]
#[serde(into = "String")]
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

    pub fn usize_at(&self, i: usize) -> Result<usize> {
        let result = self.0[i].parse()?;
        Ok(result)
    }

    pub fn to_string(&self) -> String {
        self.0.join("/")
    }

}

impl From<String> for IdPath {
    fn from(a: String) -> Self {
        IdPath(a.split('/').map(|s| s.to_owned()).collect())
    }
}

impl From<&IdPath> for String {
    fn from(a: &IdPath) -> Self {
        a.to_string()
    }
}

impl From<IdPath> for String {
    fn from(a: IdPath) -> Self {
        a.to_string()
    }
}
