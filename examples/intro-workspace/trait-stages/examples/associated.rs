mod associated_support;

use crate::associated_support::Extract;
use crate::associated_support::Holder;
use crate::associated_support::SwitchState;
use crate::associated_support::double_number;
use crate::associated_support::extract_value;

fn main() {
    // --8<-- [start:direct]
    let number = Holder { value: 120 };
    let switch = SwitchState { enabled: true };
    let amount: i32 = number.extract();
    let enabled: bool = switch.extract();
    println!("直接取出數字={amount}、布林值={enabled}");
    // --8<-- [end:direct]

    // --8<-- [start:generic]
    let amount: i32 = extract_value(Holder { value: 120 });
    let enabled: bool = extract_value(SwitchState { enabled: true });
    println!("共用函式取得數字={amount}、布林值={enabled}");
    // --8<-- [end:generic]

    let doubled = double_number(Holder { value: 120 });
    println!("數字結果乘二={doubled}");
}
