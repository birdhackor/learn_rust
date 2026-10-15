use std::fmt;
use std::io;

#[derive(Debug)]
pub enum AppError {
    EmptyQuery,
    Read(io::Error),
    Encode(serde_json::Error),
    Output(io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyQuery => write!(f, "搜尋文字不能是空字串"),
            Self::Read(error) => write!(f, "文字讀取失敗：{error}"),
            Self::Encode(error) => write!(f, "JSON 編碼失敗：{error}"),
            Self::Output(error) => write!(f, "結果輸出失敗：{error}"),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        Self::Encode(error)
    }
}

impl AppError {
    pub fn status(&self) -> u8 {
        match self {
            Self::EmptyQuery => 2,
            Self::Read(_) => 1,
            Self::Encode(_) => 1,
            Self::Output(_) => 1,
        }
    }
}
