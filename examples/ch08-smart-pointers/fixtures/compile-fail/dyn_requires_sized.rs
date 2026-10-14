trait SizedCharge: Sized {
    fn amount(&self) -> i32;
}

fn read(value: &dyn SizedCharge) -> i32 {
    value.amount()
}

fn main() {}
