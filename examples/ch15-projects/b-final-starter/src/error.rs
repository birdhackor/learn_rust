use crate::batch::ConfigError;
use crate::input::InputError;
use std::error::Error;
use std::fmt;
use std::io;

// --8<-- [start:error]
#[derive(Debug)]
#[non_exhaustive]
pub enum RunError {
    ZeroJobs,
    ZeroTimeout,
    Empty,
    Read(io::Error),
    Client(reqwest::Error),
    Signal(String),
    StopSourceClosed,
    Controller(String),
    Encode(serde_json::Error),
    Output(io::Error),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroJobs => write!(f, "jobs 必須大於零"),
            Self::ZeroTimeout => write!(f, "timeout-secs 必須大於零"),
            Self::Empty => write!(f, "輸入沒有非空白行"),
            Self::Read(_) => write!(f, "無法讀取 UTF-8 網址清單"),
            Self::Client(_) => write!(f, "無法準備 HTTP client"),
            Self::Signal(message) => write!(f, "訊號監聽失敗：{}", message.as_str()),
            Self::StopSourceClosed => write!(f, "停止來源已關閉，沒有使用者中止事件"),
            Self::Controller(message) => write!(f, "訊號工作無法正常收尾：{}", message.as_str()),
            Self::Encode(_) => write!(f, "無法編碼報告"),
            Self::Output(_) => write!(f, "無法完成報告輸出"),
        }
    }
}

impl Error for RunError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Read(error) | Self::Output(error) => Some(error),
            Self::Client(error) => Some(error),
            Self::Encode(error) => Some(error),
            _ => None,
        }
    }
}
// --8<-- [end:error]

impl From<ConfigError> for RunError {
    fn from(error: ConfigError) -> Self {
        match error {
            ConfigError::ZeroJobs => Self::ZeroJobs,
            ConfigError::Empty => Self::Empty,
        }
    }
}
impl From<InputError> for RunError {
    fn from(error: InputError) -> Self {
        match error {
            InputError::Empty => Self::Empty,
        }
    }
}
impl From<reqwest::Error> for RunError {
    fn from(error: reqwest::Error) -> Self {
        Self::Client(error)
    }
}
impl From<serde_json::Error> for RunError {
    fn from(error: serde_json::Error) -> Self {
        Self::Encode(error)
    }
}
impl From<io::Error> for RunError {
    fn from(error: io::Error) -> Self {
        Self::Output(error)
    }
}
