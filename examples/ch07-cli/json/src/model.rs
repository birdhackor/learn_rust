use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Notebook {
    pub title: String,
    pub values: Vec<i32>,
    pub archived: bool,
}
