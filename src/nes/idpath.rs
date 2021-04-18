use crate::errors::*;
use serde::{Deserialize, Serialize};
use std::convert::{From, Into};
use std::fmt;
use std::ops::RangeBounds;
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
            Err(ErrorKind::IdPathBadLength(category.to_owned(), self.to_string(), len).into())
        }
    }
    pub fn check_range<R>(&self, category: &str, range: R) -> Result<()>
    where
        R: RangeBounds<usize> + fmt::Debug,
    {
        if range.contains(&self.0.len()) {
            Ok(())
        } else {
            let range = format!("{:?}", range);
            Err(ErrorKind::IdPathBadRange(category.to_owned(), self.to_string(), range).into())
        }
    }

    pub fn prefix(&self, other: &IdPath) -> bool {
        if other.len() == 0 {
            return false;
        }
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            if a != b {
                return false;
            }
        }
        return true;
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

    pub fn last(&self) -> &str {
        let index = self.0.len() - 1;
        &self.0[index]
    }

    pub fn usize_last(&self) -> Result<usize> {
        let index = self.0.len() - 1;
        let result = self.0[index].parse()?;
        Ok(result)
    }

    pub fn set<T: ToString>(&mut self, index: isize, val: T) -> Result<()> {
        let index = if index < 0 {
            (self.0.len() as isize + index) as usize
        } else {
            index as usize
        };
        if let Some(item) = self.0.get_mut(index) {
            *item = val.to_string();
            Ok(())
        } else {
            Err(ErrorKind::IndexError(index).into())
        }
    }

    pub fn extend<T: ToString>(&self, val: T) -> Self {
        let mut ret = self.clone();
        for component in val.to_string().split('/') {
            ret.0.push(component.to_string());
        }
        ret
    }

    pub fn pop(&self) -> Self {
        let mut ret = self.clone();
        ret.0.pop();
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
