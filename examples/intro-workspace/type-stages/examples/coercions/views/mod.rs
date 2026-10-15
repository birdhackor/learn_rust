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
