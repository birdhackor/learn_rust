use crate::Inspection;
use serde::Deserialize;
use std::fmt;

// --8<-- [start:reply-model]
#[derive(Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PageRecord {
    pub title: String,
    pub revision: i32,
}

#[derive(Debug)]
pub enum ReplyError {
    Http(u16),
    BodyNotRequested,
    Json(serde_json::Error),
}

impl fmt::Display for ReplyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(status) => write!(f, "HTTP {status} 未通過，未解碼頁面資料"),
            Self::BodyNotRequested => write!(f, "沒有要求讀完整 body"),
            Self::Json(error) => write!(f, "body 不符合頁面 JSON 模型：{error}"),
        }
    }
}
// --8<-- [end:reply-model]

// --8<-- [start:reply-solution]
pub fn decode_page(inspection: &Inspection) -> Result<PageRecord, ReplyError> {
    if !inspection.passed {
        return Err(ReplyError::Http(inspection.status));
    }
    let text = match &inspection.body {
        Some(text) => text,
        None => return Err(ReplyError::BodyNotRequested),
    };
    match serde_json::from_str::<PageRecord>(text.as_str()) {
        Ok(page) => Ok(page),
        Err(error) => Err(ReplyError::Json(error)),
    }
}
// --8<-- [end:reply-solution]

#[cfg(test)]
mod tests {
    use crate::Inspection;
    use crate::reply::ReplyError;
    use crate::reply::decode_page;

    // --8<-- [start:reply-test]
    #[test]
    fn body_schema_is_separate_from_http_success_and_model_is_owned() {
        let page = {
            let inspection = Inspection {
                final_url: "https://example.org/notes".to_owned(),
                status: 200,
                passed: true,
                body: Some("{\"title\":\"筆記\",\"revision\":3}".to_owned()),
                elapsed_ms: 0,
            };
            decode_page(&inspection).expect("人工材料符合已知模型")
        };
        assert_eq!(page.title.as_str(), "筆記");
        assert_eq!(page.revision, 3);

        let mut inspection = Inspection {
            final_url: "https://example.org/notes".to_owned(),
            status: 200,
            passed: true,
            body: None,
            elapsed_ms: 0,
        };
        match decode_page(&inspection) {
            Err(ReplyError::BodyNotRequested) => {}
            other => panic!("未讀 body 不能解碼：{other:?}"),
        }
        inspection.body = Some("{\"title\":\"筆記\",\"revision\":\"3\"}".to_owned());
        match decode_page(&inspection) {
            Err(ReplyError::Json(_)) => {}
            other => panic!("HTTP 200 不保證 revision 型別：{other:?}"),
        }
        inspection.status = 404;
        inspection.passed = false;
        match decode_page(&inspection) {
            Err(ReplyError::Http(404)) => {}
            other => panic!("HTTP 未通過先拒絕，不先猜 JSON：{other:?}"),
        }
    }
    // --8<-- [end:reply-test]
}
