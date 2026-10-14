// --8<-- [start:models]
pub struct Holder<T> {
    pub value: T,
}

pub struct SwitchState {
    pub enabled: bool,
}
// --8<-- [end:models]

// --8<-- [start:extract-trait]
pub trait Extract {
    type Output;

    fn extract(self) -> Self::Output;
}
// --8<-- [end:extract-trait]

// --8<-- [start:extract-impls]
impl<T> Extract for Holder<T> {
    type Output = T;

    fn extract(self) -> Self::Output {
        self.value
    }
}

impl Extract for SwitchState {
    type Output = bool;

    fn extract(self) -> Self::Output {
        self.enabled
    }
}
// --8<-- [end:extract-impls]

// --8<-- [start:extract-generic]
pub fn extract_value<S: Extract>(source: S) -> S::Output {
    source.extract()
}
// --8<-- [end:extract-generic]

// --8<-- [start:numeric-only]
pub fn double_number<S: Extract<Output = i32>>(source: S) -> i32 {
    source.extract() * 2
}
// --8<-- [end:numeric-only]

// --8<-- [start:associated-tests]
#[cfg(test)]
mod tests {
    use crate::associated::Holder;
    use crate::associated::SwitchState;
    use crate::associated::double_number;
    use crate::associated::extract_value;

    #[test]
    fn different_sources_preserve_their_result_types() {
        let amount: i32 = extract_value(Holder { value: 120 });
        let enabled: bool = extract_value(SwitchState { enabled: false });
        assert_eq!(amount, 120);
        assert!(!enabled);
    }

    #[test]
    fn numeric_work_accepts_numeric_outputs() {
        assert_eq!(double_number(Holder { value: 0 }), 0);
        assert_eq!(double_number(Holder { value: 120 }), 240);
    }

    #[test]
    fn owned_text_survives_extraction_from_its_holder() {
        let mut text = String::new();
        text.push_str("Rust");
        let source = Holder { value: text };
        let mut taken = extract_value(source);
        taken.push_str(" 教學");
        assert_eq!(taken.as_str().len(), 11);
    }
}
// --8<-- [end:associated-tests]
