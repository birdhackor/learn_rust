use ch08_smart_pointers::dyn_charges::Bundle;
use ch08_smart_pointers::dyn_charges::Measure;
use ch08_smart_pointers::dyn_charges::RebuildCharge;

fn main() {
    let original = Bundle { price: 120 };
    let copied = original.rebuild();
    println!("具體型別可rebuild={}", copied.price);

    let value: Box<dyn RebuildCharge> = Box::new(original);
    println!("dyn仍可呼叫amount={}", value.amount());

    let measured: Box<dyn Measure<Output = i32>> = Box::new(Bundle { price: 80 });
    println!("已選定Output的dyn結果={}", measured.measure());
}
