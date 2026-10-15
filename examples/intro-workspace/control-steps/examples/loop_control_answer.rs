fn main() {
    let mut current = 0;
    let first_even = loop {
        current += 1;
        if current % 2 == 0 {
            break current;
        }
    };
    println!("第一個正偶數={first_even}");

    let mut total = 0;
    for number in 1..=5 {
        if number == 3 {
            continue;
        }
        total += number;
    }
    println!("跳過3的總和={total}");
}
