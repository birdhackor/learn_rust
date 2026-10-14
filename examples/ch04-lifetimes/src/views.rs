// --8<-- [start:single-view]
pub struct TextView<'a> {
    pub text: &'a str,
}
// --8<-- [end:single-view]

// --8<-- [start:make-view]
pub fn make_view(text: &str) -> TextView<'_> {
    TextView { text }
}
// --8<-- [end:make-view]

// --8<-- [start:view-methods]
impl<'a> TextView<'a> {
    pub fn byte_len(&self) -> usize {
        self.text.len()
    }

    pub fn text(&self) -> &str {
        self.text
    }

    pub fn original(&self) -> &'a str {
        self.text
    }
}
// --8<-- [end:view-methods]

// --8<-- [start:two-view]
pub struct TextPair<'a, 'b> {
    pub first: &'a str,
    pub second: &'b str,
}

impl<'a, 'b> TextPair<'a, 'b> {
    pub fn byte_len(&self) -> usize {
        self.first.len() + self.second.len()
    }

    pub fn first_source(&self) -> &'a str {
        self.first
    }

    pub fn second_source(&self) -> &'b str {
        self.second
    }
}
// --8<-- [end:two-view]

#[cfg(test)]
mod tests {
    use crate::views::TextPair;
    use crate::views::make_view;

    #[test]
    fn borrowed_model_keeps_its_source_content() {
        let mut source = String::new();
        source.push_str("Rust");
        let view = make_view(source.as_str());
        assert!(view.byte_len() == 4);
        assert!(view.text().len() == 4);
        assert!(view.original().len() == 4);
    }

    #[test]
    fn first_source_can_survive_the_other_source_and_model() {
        let mut first = String::new();
        first.push_str("Rust tutorial");
        let saved = {
            let mut second = String::new();
            second.push_str("note");
            let pair = TextPair {
                first: first.as_str(),
                second: second.as_str(),
            };
            assert!(pair.byte_len() == 17);
            assert!(pair.second_source().len() == 4);
            pair.first_source()
        };
        assert!(saved.len() == 13);
    }
}
