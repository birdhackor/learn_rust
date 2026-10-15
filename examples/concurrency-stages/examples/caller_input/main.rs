pub mod higher_ranked;

use crate::higher_ranked::on_caller;

fn main() {
    let value = 80;
    let result = on_caller(&value, |number: &i32| *number + 120);
    assert_eq!(result, 200);
    println!("呼叫者提供借用，結果={result}");
}
