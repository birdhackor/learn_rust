trait Charge {
    fn amount(&self) -> i32;
}

fn main() {
    let value: Box<dyn Charge> = Box::new(80);
    println!("金額={}", value.amount());
}
