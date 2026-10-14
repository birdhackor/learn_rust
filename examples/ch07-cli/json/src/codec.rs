// --8<-- [start:codec]
use crate::model::Notebook;

pub fn decode(text: &str) -> Result<Notebook, serde_json::Error> {
    serde_json::from_str::<Notebook>(text)
}

pub fn encode(notebook: &Notebook) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(notebook)
}
// --8<-- [end:codec]

// --8<-- [start:codec-tests]
#[cfg(test)]
mod tests {
    use crate::codec::decode;
    use crate::codec::encode;
    use std::fs;

    #[test]
    fn owned_model_survives_the_input_string_and_round_trip_keeps_all_fields() {
        let notebook = {
            let text =
                fs::read_to_string("fixtures/notebook.json").expect("教材固定 JSON 檔為 UTF-8");
            decode(text.as_str()).expect("教材資料符合 Notebook 模型")
        };
        assert_eq!(notebook.title.as_str(), "筆記");
        assert_eq!(notebook.values.len(), 2);
        assert_eq!(notebook.values.as_slice()[0], 80);
        assert_eq!(notebook.values.as_slice()[1], 120);
        assert!(!notebook.archived);

        let text = encode(&notebook).expect("這個模型可編成 JSON");
        let recovered = decode(text.as_str()).expect("剛編出的 JSON 符合模型");
        assert_eq!(recovered, notebook);
    }

    #[test]
    fn syntax_missing_fields_unknown_fields_and_wrong_types_are_rejected() {
        for filename in [
            "fixtures/broken-syntax.json",
            "fixtures/missing-title.json",
            "fixtures/unknown-field.json",
            "fixtures/wrong-value-type.json",
            "fixtures/number-out-of-range.json",
        ] {
            let text = fs::read_to_string(filename).expect("教材固定檔為 UTF-8");
            if let Ok(unexpected) = decode(text.as_str()) {
                panic!("這份資料應因語法或模型限制失敗：{filename}，卻讀到 {unexpected:?}");
            }
        }
    }

    #[test]
    fn escaping_keeps_the_original_title_content() {
        let text = fs::read_to_string("fixtures/quoted-title.json").expect("教材固定檔為 UTF-8");
        let original = decode(text.as_str()).expect("已知 JSON 的跳脫形式合法");
        let encoded = encode(&original).expect("此模型可編成 JSON");
        let recovered = decode(encoded.as_str()).expect("編出的 JSON 可讀回");
        assert_eq!(recovered, original);
    }
}
// --8<-- [end:codec-tests]
