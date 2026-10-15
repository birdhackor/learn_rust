use crate::Options;
use crate::SearchError;
use std::fs;
use std::path::Path;

// --8<-- [start:hit]
#[derive(Debug, PartialEq)]
pub struct Hit {
    pub line_number: usize,
    pub text: String,
}
// --8<-- [end:hit]

// --8<-- [start:search]
fn validate_query(query: &str) -> Result<(), SearchError> {
    if query.is_empty() {
        Err(SearchError::EmptyQuery)
    } else {
        Ok(())
    }
}

/// 搜尋非空連續片段，保留原行號與文字；最多交回options所選上限。
///
/// 零筆符合仍是Ok。空query回EmptyQuery。ignore_case只折疊ASCII。
pub fn search(query: &str, contents: &str, options: &Options) -> Result<Vec<Hit>, SearchError> {
    validate_query(query)?;
    let needle = if options.ignore_case() {
        query.to_ascii_lowercase()
    } else {
        query.to_owned()
    };
    let mut hits = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        let candidate = if options.ignore_case() {
            line.to_ascii_lowercase()
        } else {
            line.to_owned()
        };
        if candidate.as_str().contains(needle.as_str()) {
            hits.push(Hit {
                line_number: index + 1,
                text: line.to_owned(),
            });
            if hits.len() == options.limit().get() {
                break;
            }
        }
    }
    Ok(hits)
}
// --8<-- [end:search]

// --8<-- [start:read-matches]
/// 先驗證query，再讀完整UTF-8檔案，成功結果擁有原行文字。
pub fn read_matches(
    query: &str,
    filename: &Path,
    options: &Options,
) -> Result<Vec<Hit>, SearchError> {
    validate_query(query)?;
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(error) => return Err(SearchError::Read(error)),
    };
    search(query, contents.as_str(), options)
}
// --8<-- [end:read-matches]
