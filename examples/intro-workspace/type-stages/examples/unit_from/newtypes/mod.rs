pub struct Wrapped<T>(pub T);

impl<T> From<T> for Wrapped<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Wrapped<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}
