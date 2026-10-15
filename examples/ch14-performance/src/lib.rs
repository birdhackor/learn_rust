// --8<-- [start:counts]
pub fn count_borrowed(query: &str, lines: &[String]) -> Result<usize, String> {
    if query.is_empty() {
        return Err(String::from("搜尋文字不能是空字串"));
    }
    let mut count = 0;
    for line in lines {
        if line.as_str().contains(query) {
            count += 1;
        }
    }
    Ok(count)
}

pub fn count_cloned(query: &str, lines: &[String]) -> Result<usize, String> {
    if query.is_empty() {
        return Err(String::from("搜尋文字不能是空字串"));
    }
    let mut count = 0;
    for line in lines {
        let candidate: String = line.clone();
        if candidate.as_str().contains(query) {
            count += 1;
        }
    }
    Ok(count)
}
// --8<-- [end:counts]

// --8<-- [start:workload]
pub fn make_workload(rows: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for index in 0..rows {
        let text = if index % 4 == 0 {
            "Rust Rust notes"
        } else {
            "Python notes"
        };
        lines.push(String::from(text));
    }
    lines
}
// --8<-- [end:workload]
