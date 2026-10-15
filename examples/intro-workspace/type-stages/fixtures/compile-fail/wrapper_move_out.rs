use std::ops::Deref;

struct Wrapper<T> {
    value: T,
}

impl<T> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let wrapped = Wrapper { value: text };
    let taken = *wrapped;
    println!("{}", taken.as_str());
}
