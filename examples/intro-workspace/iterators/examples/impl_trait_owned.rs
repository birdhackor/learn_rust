// --8<-- [start:parameter]
fn evaluate(task: impl Fn(i32) -> i32, value: i32) -> i32 {
    task(value)
}
// --8<-- [end:parameter]

// --8<-- [start:owned]
fn make_adder(base: i32) -> impl Fn(i32) -> i32 {
    move |value| value + base
}
// --8<-- [end:owned]

// --8<-- [start:branches]
fn make_adjuster(subtract: bool, base: i32) -> impl Fn(i32) -> i32 {
    move |value| {
        if subtract { value - base } else { value + base }
    }
}
// --8<-- [end:branches]

fn main() {
    println!("參數位置接受工作={}", evaluate(|value| value * 2, 3));
    let add = make_adder(10);
    println!("工廠交回工作={}", add(3));
    println!("保留工作再呼叫={}", add(4));

    let subtract = make_adjuster(true, 10);
    let increase = make_adjuster(false, 10);
    println!(
        "同一個 closure body 選分支={}、{}",
        subtract(30),
        increase(30)
    );
}
