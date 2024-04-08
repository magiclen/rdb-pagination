use std::{
    cmp::Ordering,
    fmt,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    ops::Deref,
};

/// Enum representing the name of a table or a column.
#[derive(Debug, Clone)]
pub enum Name {
    Static(&'static str),
    Dynamic(String),
}

impl From<&'static str> for Name {
    #[inline]
    fn from(value: &'static str) -> Self {
        Self::Static(value)
    }
}

impl From<String> for Name {
    #[inline]
    fn from(value: String) -> Self {
        Self::Dynamic(value)
    }
}

impl PartialEq for Name {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl Eq for Name {}

impl PartialOrd for Name {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Name {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl Hash for Name {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}

impl AsRef<str> for Name {
    #[inline]
    fn as_ref(&self) -> &str {
        match self {
            Self::Static(s) => s,
            Self::Dynamic(s) => s.as_str(),
        }
    }
}

impl Deref for Name {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl Display for Name {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.as_ref(), f)
    }
}

/// The name of a table.
pub type TableName = Name;
/// The name of a column.
pub type ColumnName = Name;
/// The name of a table and the name of a column inside that table.
pub type TableColumn = (TableName, ColumnName);
