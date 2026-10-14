// --8<-- [start:borrowed]
pub struct Borrowed<'a, T: ?Sized + 'a> {
    pub value: &'a T,
}
// --8<-- [end:borrowed]

// --8<-- [start:static-bound]
pub fn require_static<T: 'static>(value: T) -> T {
    value
}
// --8<-- [end:static-bound]

// --8<-- [start:choose-label]
pub fn choose_label(candidate: &str, accept: bool) -> &str {
    if accept { candidate } else { "Rust" }
}
// --8<-- [end:choose-label]

#[cfg(test)]
mod tests {
    use crate::lifetime_bounds::Borrowed;
    use crate::lifetime_bounds::choose_label;
    use crate::lifetime_bounds::require_static;

    #[test]
    fn a_static_literal_can_fill_a_shorter_borrowed_result() {
        let mut candidate = String::new();
        candidate.push_str("Python");
        assert_eq!(choose_label(candidate.as_str(), true), "Python");
        assert_eq!(choose_label(candidate.as_str(), false), "Rust");
    }

    #[test]
    fn owning_a_string_satisfies_static_without_cloning_it() {
        let mut text = String::new();
        text.push_str("Rust");
        let boxed = require_static(Box::new(text));
        let mut taken = *boxed;
        taken.push_str(" 教學");
        assert_eq!(taken.as_str(), "Rust 教學");
    }

    #[test]
    fn a_borrowed_type_can_fit_a_local_bound() {
        let mut text = String::new();
        text.push_str("Rust");
        let reference = text.as_str();
        let nested = Borrowed { value: &reference };
        assert_eq!(*nested.value, "Rust");
    }
}
