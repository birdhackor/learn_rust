trait Charge {
    fn amount(&self) -> i32;
}

fn meets_threshold<T>(value: &T, threshold: i32) -> bool {
    value.amount() >= threshold
}

fn main() {}
