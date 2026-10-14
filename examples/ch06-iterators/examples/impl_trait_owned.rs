use ch06_iterators::opaque::evaluate;
use ch06_iterators::opaque::make_adder;
use ch06_iterators::opaque::make_adjuster;

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
