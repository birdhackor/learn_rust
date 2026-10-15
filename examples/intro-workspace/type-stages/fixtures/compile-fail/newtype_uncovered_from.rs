struct Wrapped<T>(T);

impl<T> From<Wrapped<T>> for T {
    fn from(value: Wrapped<T>) -> Self {
        value.0
    }
}

fn main() {}
