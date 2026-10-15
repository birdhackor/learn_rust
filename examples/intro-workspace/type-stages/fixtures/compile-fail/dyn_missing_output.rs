trait Measure {
    type Output;

    fn measure(&self) -> Self::Output;
}

fn read(_: &dyn Measure) {}

fn main() {}
