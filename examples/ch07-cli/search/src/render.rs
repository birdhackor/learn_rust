// --8<-- [start:render]
use crate::error::AppError;
use crate::model::Report;

pub fn render(report: &Report, json: bool) -> Result<String, AppError> {
    if json {
        let mut text = serde_json::to_string_pretty(report)?;
        text.push('\n');
        Ok(text)
    } else {
        let mut text = String::new();
        for record in report.matches.as_slice() {
            let row = format!("{}:{}\n", record.line_number, record.text.as_str());
            text.push_str(row.as_str());
        }
        Ok(text)
    }
}
// --8<-- [end:render]

#[cfg(test)]
mod tests {
    use crate::model::Report;
    use crate::render::render;
    use crate::search::search;

    #[test]
    fn plain_text_keeps_the_line_number_and_original_line() {
        let report = search("Rust", "Rust\nPython\nRust \"筆記\"\n", false).expect("搜尋文字非空");
        assert_eq!(
            render(&report, false).expect("文字排版").as_str(),
            "1:Rust\n3:Rust \"筆記\"\n"
        );
    }

    #[test]
    fn json_round_trip_keeps_every_field_and_empty_matches_are_valid() {
        let report = search("Rust", "Rust \"筆記\"\n", false).expect("搜尋文字非空");
        let text = render(&report, true).expect("模型可編碼");
        let recovered = serde_json::from_str::<Report>(text.as_str()).expect("模型可讀回");
        assert_eq!(recovered, report);
        let empty = search("Rust", "Python\n", false).expect("搜尋文字非空");
        assert_eq!(render(&empty, false).expect("文字排版").as_str(), "");
        let encoded = render(&empty, true).expect("空結果同樣可編碼");
        let recovered = serde_json::from_str::<Report>(encoded.as_str()).expect("模型可讀回");
        assert_eq!(recovered, empty);
    }
}
