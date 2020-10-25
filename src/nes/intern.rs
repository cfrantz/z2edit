use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::From;

struct InternTable {
    pub index: HashMap<usize, String>,
    pub string: HashMap<String, usize>,
}

static mut INTERN: Option<InternTable> = None;

impl InternTable {
    pub fn new() -> InternTable {
        InternTable {
            index: HashMap::new(),
            string: HashMap::new(),
        }
    }

    pub fn contains(&self, v: &str) -> bool {
        self.string.contains_key(v)
    }

    pub fn insert(&mut self, v: &str) -> usize {
        if self.contains(v) {
            *self.string.get(v).unwrap()
        } else {
            let n = self.index.len();
            self.index.insert(n, v.to_owned());
            self.string.insert(v.to_owned(), n);
            n
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "String")]
#[serde(into = "String")]
pub struct Istr(usize);

impl From<String> for Istr {
    fn from(a: String) -> Self {
        Istr::new(&a)
    }
}

impl From<Istr> for String {
    fn from(a: Istr) -> Self {
        a.as_str().to_owned()
    }
}

impl Istr {
    pub fn new(s: &str) -> Istr {
        let intern = unsafe {
            if INTERN.is_none() {
                INTERN = Some(InternTable::new());
            }
            INTERN.as_mut().unwrap()
        };
        Istr(intern.insert(s))
    }

    pub fn as_str(&self) -> &'static str {
        let intern = unsafe {
            if INTERN.is_none() {
                panic!("InternTable accessed before creation.");
            }
            INTERN.as_mut().unwrap()
        };
        intern.index.get(&self.0).unwrap()
    }
}
