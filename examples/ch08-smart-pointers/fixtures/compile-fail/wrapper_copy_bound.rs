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

fn copy_value<T: Copy>(value: T) -> T {
    value
}

fn main() {
    let wrapped = Wrapper { value: 80 };
    let copied = copy_value(wrapped);
    println!("{}", copied.value);
}
