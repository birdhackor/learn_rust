use ch08_smart_pointers::newtypes::PriceFn;
use ch08_smart_pointers::newtypes::SecondAlias;
use ch08_smart_pointers::newtypes::alias_distance;
use ch08_smart_pointers::newtypes::price_fn;

fn main() {
    let elapsed: SecondAlias = 80;
    let distance = alias_distance(elapsed);
    println!("把秒數誤傳成路程，編譯器仍接受={distance}");

    let short: PriceFn = price_fn(true);
    let original: Box<dyn Fn(i32) -> i32> = short;
    println!("完整型別收到同一closure，80調整為={}", original(80));
}
