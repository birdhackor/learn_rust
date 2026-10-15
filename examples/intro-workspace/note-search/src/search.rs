// --8<-- [start:search]
use crate::error::AppError;
use crate::model::Match;
use crate::model::Report;

pub fn validate_query(query: &str) -> Result<(), AppError> {
    if query.is_empty() {
        Err(AppError::EmptyQuery)
    } else {
        Ok(())
    }
}

pub fn search(query: &str, contents: &str, ignore_case: bool) -> Result<Report, AppError> {
    validate_query(query)?;
    let search_query = if ignore_case {
        query.to_ascii_lowercase()
    } else {
        query.to_owned()
    };
    let mut matches = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        let candidate = if ignore_case {
            line.to_ascii_lowercase()
        } else {
            line.to_owned()
        };
        if candidate.as_str().contains(search_query.as_str()) {
            matches.push(Match {
                line_number: index + 1,
                text: line.to_owned(),
            });
        }
    }
    Ok(Report {
        query: query.to_owned(),
        ignore_case,
        matches,
    })
}
// --8<-- [end:search]

// --8<-- [start:search-tests]
#[cfg(test)]
mod tests {
    use crate::error::AppError;
    use crate::model::Match;
    use crate::search::search;

    #[test]
    fn case_sensitive_and_ascii_flag_have_distinct_answers() {
        let contents = "Rust 很清楚\nPython\nrust 也有資料\n中有 Rust\n";
        let ordinary = search("Rust", contents, false).expect("搜尋文字非空");
        assert_eq!(
            ordinary.matches,
            vec![
                Match {
                    line_number: 1,
                    text: "Rust 很清楚".to_owned(),
                },
                Match {
                    line_number: 4,
                    text: "中有 Rust".to_owned(),
                },
            ]
        );
        let folded = search("RUST", contents, true).expect("搜尋文字非空");
        assert_eq!(folded.matches.len(), 3);
        assert_eq!(folded.matches.as_slice()[1].line_number, 3);
        assert_eq!(folded.matches.as_slice()[1].text.as_str(), "rust 也有資料");
    }

    #[test]
    fn ascii_conversion_does_not_fold_non_ascii_letters() {
        let report = search("Ä", "Ä\nä\n", true).expect("搜尋文字非空");
        assert_eq!(report.matches.len(), 1);
        assert_eq!(report.matches.as_slice()[0].text.as_str(), "Ä");
    }

    #[test]
    fn line_numbers_count_blank_lines_and_crlf_is_removed() {
        let report = search("Rust", "Python\r\n\r\nRust\r\n", false).expect("搜尋文字非空");
        assert_eq!(report.matches.as_slice()[0].line_number, 3);
        assert_eq!(report.matches.as_slice()[0].text.as_str(), "Rust");
        assert_eq!(
            search("Rust", "", false)
                .expect("搜尋文字非空")
                .matches
                .len(),
            0
        );
    }

    #[test]
    fn report_owns_data_after_inputs_end_and_keeps_original_spelling() {
        let report = {
            let query = "RUST".to_owned();
            let contents = "Rust \"筆記\"\n".to_owned();
            search(query.as_str(), contents.as_str(), true).expect("搜尋文字非空")
        };
        assert_eq!(report.query.as_str(), "RUST");
        assert_eq!(report.matches.as_slice()[0].text.as_str(), "Rust \"筆記\"");
    }

    #[test]
    fn empty_query_is_rejected_even_when_the_contents_are_empty() {
        match search("", "", false) {
            Err(AppError::EmptyQuery) => {}
            other => panic!("空搜尋應是用法錯誤，卻得到 {other:?}"),
        }
    }
}
// --8<-- [end:search-tests]
