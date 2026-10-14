fn main() {
    let prices = [120, 80, 200];
    let mut total = 0;
    for price in prices {
        total += price;
    }
    println!("for的總和={total}");

    let mut count = 0;
    let mut sum = 0;
    while count < 3 {
        count += 1;
        sum += count;
    }
    println!("while的總和={sum}");

    let mut current = 0;
    let found = loop {
        current += 1;
        if current >= 3 {
            break current;
        }
    };
    println!("loop找到的值={found}");

    let mut sum = 0;
    for number in 0..=3 {
        if number == 2 {
            continue;
        }
        sum += number;
    }
    println!("跳過2的總和={sum}");
}
