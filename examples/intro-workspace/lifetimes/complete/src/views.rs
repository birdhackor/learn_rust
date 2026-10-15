pub struct TextView<'a> {
    pub text: &'a str,
}

pub fn make_view(text: &str) -> TextView<'_> {
    TextView { text }
}

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

// --8<-- [start:text-pair]
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
// --8<-- [end:text-pair]

#[cfg(test)]
mod tests {
    use crate::views::TextPair;
    use crate::views::make_view;

    #[test]
    fn original_borrow_can_survive_the_model() {
        let mut source = String::new();
        source.push_str("Rust");
        let saved = {
            let view = make_view(source.as_str());
            assert_eq!(view.byte_len(), 4);
            assert_eq!(view.text(), "Rust");
            view.original()
        };
        assert_eq!(saved, "Rust");
    }

    #[test]
    fn first_source_can_survive_other_source_and_model() {
        let mut first = String::new();
        first.push_str("Rust tutorial");
        let saved = {
            let mut second = String::new();
            second.push_str("note");
            let pair = TextPair {
                first: first.as_str(),
                second: second.as_str(),
            };
            assert_eq!(pair.byte_len(), 17);
            assert_eq!(pair.second_source(), "note");
            pair.first_source()
        };
        assert_eq!(saved, "Rust tutorial");
    }
}
