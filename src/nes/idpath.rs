use crate::errors::*;
use serde::{Deserialize, Serialize};
use std::convert::{From, Into};
use std::fmt;
use std::ops::Range;
use std::string::ToString;

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
    pub fn check_range(&self, category: &str, range: Range<usize>) -> Result<()> {
        if range.contains(&self.0.len()) {
            Ok(())
        } else {
            Err(ErrorKind::IdPathBadRange(category.to_owned(), range).into())
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

    pub fn extend<T: ToString>(&self, val: T) -> Self {
        let mut ret = self.clone();
        for component in val.to_string().split('/') {
            ret.0.push(component.to_string());
        }
        ret
    }
}

impl From<&str> for IdPath {
    fn from(a: &str) -> Self {
        IdPath(a.split('/').map(|s| s.to_owned()).collect())
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

impl fmt::Display for IdPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join("/"))
    }
}

#[macro_export]
macro_rules! idpath {
    () => (
        $crate::nes::IdPath(vec![])
    );

    ($($x:expr),+ $(,)?) => (
        $crate::nes::IdPath(vec![$($x.to_string()),+])
    );
}
