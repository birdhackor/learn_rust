use ch08_smart_pointers::chain::Chain;
use ch08_smart_pointers::chain::into_total;

fn main() {
    let chain = Chain::Link {
        value: 80,
        next: Box::new(Chain::Link {
            value: 120,
            next: Box::new(Chain::End),
        }),
    };
    let total = into_total(chain);
    println!("沿連結加總={total}");
    println!("空連結加總={}", into_total(Chain::End));
}
