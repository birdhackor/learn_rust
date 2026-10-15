trait Rebuild {
    fn amount(&self) -> i32;

    fn rebuild(&self) -> Self
    where
        Self: Sized;
}

struct Bundle {
    price: i32,
}

impl Rebuild for Bundle {
    fn amount(&self) -> i32 {
        self.price
    }

    fn rebuild(&self) -> Self {
        Bundle { price: self.price }
    }
}

fn main() {
    let value: Box<dyn Rebuild> = Box::new(Bundle { price: 120 });
    let copied = value.rebuild();
    println!("金額={}", copied.amount());
}
