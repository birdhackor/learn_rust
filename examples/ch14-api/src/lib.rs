// 有效搜尋設定、純文字搜尋與保留原因的公開錯誤介面。

// --8<-- [start:exports]
mod errors;
mod limits;
mod options;
mod search;

pub use errors::SearchError;
pub use limits::BatchLimit;
pub use limits::LimitError;
pub use options::Options;
pub use search::Hit;
pub use search::read_matches;
pub use search::search;

#[cfg(feature = "text")]
mod text;
#[cfg(feature = "text")]
pub use text::render_text;
// --8<-- [end:exports]
