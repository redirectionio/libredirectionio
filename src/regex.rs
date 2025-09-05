use std::{fmt::Display, hash::Hash, sync::Arc};

use regex::{Regex, RegexBuilder};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct LazyRegex {
    pub(crate) original: String,
    pub(crate) regex: String,
    pub(crate) compiled: Option<Arc<Regex>>,
    pub(crate) ignore_case: bool,
}

impl Serialize for LazyRegex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.original.as_str())
    }
}

impl Eq for LazyRegex {}

impl PartialEq for LazyRegex {
    fn eq(&self, other: &Self) -> bool {
        self.original == other.original && self.ignore_case == other.ignore_case
    }
}

impl Ord for LazyRegex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.original.cmp(&other.original)
    }
}

impl PartialOrd for LazyRegex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.original.cmp(&other.original))
    }
}

impl Hash for LazyRegex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.original.hash(state);
        self.ignore_case.hash(state);
    }
}

impl Display for LazyRegex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.original)
    }
}

impl LazyRegex {
    #[cfg(feature = "router")]
    pub fn new(regex: String, ignore_case: bool) -> LazyRegex {
        LazyRegex {
            regex: regex.clone(),
            original: regex,
            compiled: None,
            ignore_case,
        }
    }

    #[cfg(feature = "router")]
    pub fn new_node(regex: String, ignore_case: bool) -> LazyRegex {
        LazyRegex {
            regex: if regex.is_empty() {
                ".*".to_string()
            } else {
                ["^", regex.as_str()].join("")
            },
            original: regex,
            compiled: None,
            ignore_case,
        }
    }

    pub fn new_leaf(regex: &str, ignore_case: bool) -> LazyRegex {
        LazyRegex {
            regex: ["^", regex, "$"].join(""),
            original: regex.to_string(),
            compiled: None,
            ignore_case,
        }
    }

    #[cfg(feature = "router")]
    pub fn is_match(&self, value: &str) -> bool {
        match &self.compiled {
            Some(regex) => regex.is_match(value),
            None => {
                if self.original.is_empty() {
                    true
                } else {
                    match self.create_regex() {
                        None => false,
                        Some(regex) => regex.is_match(value),
                    }
                }
            }
        }
    }

    pub fn regex(&self) -> Option<Arc<Regex>> {
        match &self.compiled {
            Some(regex) => Some(regex.clone()),
            None => self.create_regex(),
        }
    }

    pub fn create_regex(&self) -> Option<Arc<Regex>> {
        match RegexBuilder::new(self.regex.as_str()).case_insensitive(self.ignore_case).build() {
            Ok(regex) => Some(Arc::new(regex)),
            Err(e) => {
                tracing::error!("cannot create regex: {:?}", e);

                None
            }
        }
    }

    pub fn compile(&self) -> Self {
        let compiled = self.create_regex();

        LazyRegex {
            regex: self.regex.clone(),
            original: self.original.clone(),
            compiled,
            ignore_case: self.ignore_case,
        }
    }
}
