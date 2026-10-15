trait Rebuild {
    fn rebuild(&self) -> Self;
}

fn read(_: &dyn Rebuild) {}

fn main() {}
