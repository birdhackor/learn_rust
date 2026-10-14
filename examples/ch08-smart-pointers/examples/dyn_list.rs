use ch08_smart_pointers::dyn_charges::Bundle;
use ch08_smart_pointers::dyn_charges::Charge;
use ch08_smart_pointers::dyn_charges::KnownCharge;
use ch08_smart_pointers::dyn_charges::Order;
use ch08_smart_pointers::dyn_charges::total;

struct Shipping {
    fee: i32,
}

impl Charge for Shipping {
    fn amount(&self) -> i32 {
        self.fee
    }
}

fn main() {
    let known: Vec<KnownCharge> = vec![
        KnownCharge::Order(Order {
            unit_price: 80,
            count: 1,
        }),
        KnownCharge::Bundle(Bundle { price: 120 }),
    ];
    let mut known_sum = 0;
    let mut index = 0;
    while index < known.len() {
        known_sum += known.as_slice()[index].amount();
        index += 1;
    }
    println!("已知enum種類的總額={known_sum}");

    let values: Vec<Box<dyn Charge>> = vec![
        Box::new(Order {
            unit_price: 80,
            count: 1,
        }),
        Box::new(Bundle { price: 120 }),
    ];
    println!("同一批資料用dyn的總額={}", total(values.as_slice()));

    let mut extended = values;
    extended.push(Box::new(Shipping { fee: 20 }));
    println!("使用者新增運費後總額={}", total(extended.as_slice()));
}
