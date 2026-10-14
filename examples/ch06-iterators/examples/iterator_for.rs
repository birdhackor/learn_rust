use ch06_iterators::streams::count_down;

fn main() {
    let stream = count_down(3);
    let mut total = 0;
    for value in stream {
        println!("for 取得={value}");
        total += value;
    }
    println!("倒數合計={total}");
    assert_eq!(total, 6);
}
