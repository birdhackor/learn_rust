trait GenericCharge {
    fn amount_with<T>(&self, value: T) -> i32;
}

fn read(_: &dyn GenericCharge) {}

fn main() {}
