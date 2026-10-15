use crate::Hit;

// --8<-- [start:text]
pub fn render_text(hits: &[Hit]) -> String {
    let mut text = String::new();
    for hit in hits {
        let row = format!("{}:{}\n", hit.line_number, hit.text.as_str());
        text.push_str(row.as_str());
    }
    text
}
// --8<-- [end:text]
