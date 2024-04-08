use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[doc(hidden)]
#[derive(Debug, Clone)]
pub enum OrderOptionError {
    TableNotRecognized,
    TableColumnDuplicate,
}

impl Display for OrderOptionError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::TableNotRecognized => {
                f.write_str("table has not been set, perhaps you want to join it")
            },
            Self::TableColumnDuplicate => {
                f.write_str("the table and the column have been add before, which is nonsense")
            },
        }
    }
}

impl Error for OrderOptionError {}
