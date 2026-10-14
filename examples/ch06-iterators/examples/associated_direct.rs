use ch06_iterators::associated::Extract;
use ch06_iterators::associated::Holder;
use ch06_iterators::associated::SwitchState;

fn main() {
    let number = Holder { value: 120 };
    let switch = SwitchState { enabled: true };
    let amount: i32 = number.extract();
    let enabled: bool = switch.extract();
    println!("盒子交出數字={amount}");
    println!("開關狀態交出布林值={enabled}");
}
