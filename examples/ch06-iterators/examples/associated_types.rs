use ch06_iterators::associated::Holder;
use ch06_iterators::associated::SwitchState;
use ch06_iterators::associated::double_number;
use ch06_iterators::associated::extract_value;

fn main() {
    let amount: i32 = extract_value(Holder { value: 120 });
    let enabled: bool = extract_value(SwitchState { enabled: true });
    let doubled = double_number(Holder { value: 120 });
    println!("同一函式取得數字={amount}、布林值={enabled}");
    println!("數字結果乘二={doubled}");
}
