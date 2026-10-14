use std::ops::Deref;
use std::ops::DerefMut;

// --8<-- [start:wrapper]
pub struct Wrapper<T> {
    pub value: T,
}
// --8<-- [end:wrapper]

// --8<-- [start:deref]
impl<T> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
// --8<-- [end:deref]

// --8<-- [start:deref-mut]
impl<T> DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
// --8<-- [end:deref-mut]

// --8<-- [start:into-inner]
impl<T> Wrapper<T> {
    pub fn into_inner(self) -> T {
        self.value
    }
}
// --8<-- [end:into-inner]

#[cfg(test)]
mod tests {
    use crate::views::Wrapper;

    fn text_bytes(text: &str) -> usize {
        text.len()
    }

    fn add_one(value: &mut i32) {
        *value += 1;
    }

    #[test]
    fn mutable_coercion_changes_the_original_field() {
        let mut wrapped = Wrapper { value: 80 };
        add_one(&mut wrapped);
        assert_eq!(wrapped.value, 81);
        assert_eq!(*wrapped, 81);
    }

    #[test]
    fn borrowing_preserves_owned_text_for_later_extraction() {
        let mut text = String::new();
        text.push_str("Rust");
        let wrapped = Wrapper { value: text };
        assert_eq!(text_bytes(&wrapped), 4);
        let mut taken = wrapped.into_inner();
        taken.push_str(" 教學");
        assert_eq!(taken.as_str(), "Rust 教學");
    }
}
