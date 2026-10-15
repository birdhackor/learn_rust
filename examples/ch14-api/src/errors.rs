use std::error::Error;
use std::fmt;
use std::io;

// --8<-- [start:search-error]
#[derive(Debug)]
#[non_exhaustive]
pub enum SearchError {
    EmptyQuery,
    Read(io::Error),
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyQuery => write!(f, "搜尋文字不能是空字串"),
            Self::Read(_) => write!(f, "搜尋文字檔讀取失敗"),
        }
    }
}

impl Error for SearchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::EmptyQuery => None,
            Self::Read(error) => Some(error),
        }
    }
}
// --8<-- [end:search-error]
