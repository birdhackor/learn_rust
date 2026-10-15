// --8<-- [start:file-error]
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum NotebookError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for NotebookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "文字檔讀取失敗：{error}"),
            Self::Json(error) => write!(f, "JSON 處理失敗：{error}"),
        }
    }
}

impl From<io::Error> for NotebookError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for NotebookError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}
// --8<-- [end:file-error]

// --8<-- [start:read-notebook]
use crate::json_core::codec::decode;
use crate::json_core::model::Notebook;
use std::fs;

pub fn read_notebook(filename: &str) -> Result<Notebook, NotebookError> {
    let text = fs::read_to_string(filename)?;
    let notebook = decode(text.as_str())?;
    Ok(notebook)
}
// --8<-- [end:read-notebook]

#[cfg(test)]
mod tests {
    use crate::json_files::NotebookError;
    use crate::json_files::read_notebook;
    use std::io;

    #[test]
    fn filesystem_and_json_errors_stay_distinct() {
        match read_notebook("fixtures/missing.json") {
            Err(NotebookError::Io(error)) => {
                assert_eq!(error.kind(), io::ErrorKind::NotFound);
            }
            _ => panic!("不存在的檔案應是 I/O 原因"),
        }
        match read_notebook("fixtures/broken-syntax.json") {
            Err(NotebookError::Json(_)) => {}
            _ => panic!("文字已讀到，JSON 語法卻應失敗"),
        }
        match read_notebook("fixtures/not-utf8.bin") {
            Err(NotebookError::Io(error)) => {
                assert_eq!(error.kind(), io::ErrorKind::InvalidData);
            }
            _ => panic!("UTF-8 不合法應在文字讀取步驟失敗"),
        }
    }
}
