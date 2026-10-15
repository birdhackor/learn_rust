// --8<-- [start:model]
struct LabelledOrder {
    label: String,
    unit_price: i32,
    count: i32,
}

impl LabelledOrder {
    fn new(label: String, unit_price: i32, count: i32) -> Self {
        Self {
            label,
            unit_price,
            count,
        }
    }

    fn subtotal(&self) -> i32 {
        self.unit_price * self.count
    }

    fn add_count(&mut self, additional: i32) {
        self.count += additional;
    }

    fn append_label(&mut self, part: &str) {
        self.label.push_str(part);
    }

    fn into_label(self) -> String {
        self.label
    }
}
// --8<-- [end:model]

fn main() {
    let mut label = String::new();
    label.push_str("訂單A");
    let mut order = LabelledOrder::new(label, 80, 2);
    assert_eq!(order.subtotal(), 160);
    order.add_count(1);
    order.append_label("：已備妥");
    assert_eq!(order.subtotal(), 240);
    assert_eq!(order.count, 3);
    println!(
        "標籤={}，商品小計={}",
        order.label.as_str(),
        order.subtotal()
    );

    let saved_label = order.into_label();
    println!("接手模型後留下的文字={}", saved_label.as_str());
}
