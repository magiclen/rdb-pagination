use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[doc(hidden)]
#[derive(Debug, Clone)]
pub enum JoinError {
    PrimaryDuplicate,
    ForeignNotFound,
}

impl Display for JoinError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::PrimaryDuplicate => f.write_str(
                "primary has been set, perhaps you want to use an alias table name instead?",
            ),
            Self::ForeignNotFound => {
                f.write_str("foreign has not been set, you need to join it first")
            },
        }
    }
}

impl Error for JoinError {}
