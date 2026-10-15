pub fn same(text: &str) -> &str {
    text
}

pub fn longer<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}

pub fn first_text<'a>(first: &'a str, _: &str) -> &'a str {
    first
}

pub fn copy_text(text: &str) -> String {
    let mut copied = String::new();
    copied.push_str(text);
    copied
}

pub fn same_placeholder(text: &'_ str) -> &'_ str {
    text
}

pub fn fallback<'a, 'b: 'a>(preferred: Option<&'a str>, backup: &'b str) -> &'a str {
    match preferred {
        Some(text) => text,
        None => backup,
    }
}

// --8<-- [start:first-nonempty]
pub fn first_nonempty<'a>(first: &'a str, second: &'a str) -> &'a str {
    match first.len() {
        0 => second,
        _ => first,
    }
}
// --8<-- [end:first-nonempty]

#[cfg(test)]
mod tests {
    use crate::source::copy_text;
    use crate::source::fallback;
    use crate::source::first_nonempty;
    use crate::source::longer;

    #[test]
    fn either_input_can_supply_the_result() {
        assert!(longer("Rust", "Python").len() == 6);
        assert!(longer("Python", "Rust").len() == 6);
        assert!(longer("", "Rust").len() == 4);
        assert!(longer("Rust", "").len() == 4);
    }

    #[test]
    fn owned_result_stays_independent_after_source_changes() {
        let mut original = String::new();
        original.push_str("Rust");
        let saved = copy_text(original.as_str());
        original.push_str(" tutorial");
        assert!(saved.as_str().len() == 4);
        assert!(original.as_str().len() == 13);
    }

    #[test]
    fn fallback_distinguishes_present_empty_text_from_missing() {
        let empty_length = fallback(Some(""), "backup").len();
        assert!(empty_length == 0);
        assert!(fallback(None, "backup").len() == 6);
    }

    #[test]
    fn first_nonempty_uses_second_only_for_empty_input() {
        assert!(first_nonempty("", "Rust").len() == 4);
        assert!(first_nonempty("Rust", "Python").len() == 4);
        let empty_length = first_nonempty("", "").len();
        assert!(empty_length == 0);
    }
}
