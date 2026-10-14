use std::fmt;
use std::io;

#[derive(Debug)]
pub enum AppError {
    Url(url::ParseError),
    Scheme(String),
    Client(reqwest::Error),
    Request(reqwest::Error),
    Body { status: u16, error: reqwest::Error },
    Output(io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Url(error) => write!(f, "URL 無法解析：{error}"),
            Self::Scheme(scheme) => write!(f, "只接受 http／https，收到 scheme：{scheme}"),
            Self::Client(error) => write!(f, "HTTP client 建立失敗：{error}"),
            Self::Request(error) => {
                if error.is_timeout() {
                    write!(f, "取得 final response 逾時：{error}")
                } else {
                    write!(f, "未取得 final response：{error}")
                }
            }
            Self::Body { status, error } => {
                if error.is_timeout() {
                    write!(f, "已收到 HTTP {status}，但讀取 body 逾時：{error}")
                } else {
                    write!(f, "已收到 HTTP {status}，但讀取 body 失敗：{error}")
                }
            }
            Self::Output(error) => write!(f, "結果輸出失敗：{error}"),
        }
    }
}

impl AppError {
    pub fn status(&self) -> u8 {
        match self {
            Self::Url(_) | Self::Scheme(_) => 2,
            Self::Client(_) | Self::Request(_) | Self::Body { .. } | Self::Output(_) => 1,
        }
    }
}
