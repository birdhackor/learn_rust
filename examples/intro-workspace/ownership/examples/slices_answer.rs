fn sum_values(values: &[i32]) -> i32 {
    let mut total = 0;
    for value in values {
        total += *value;
    }
    total
}

// --8<-- [start:increase]
fn increase(values: &mut [i32], amount: i32) {
    for value in values {
        *value += amount;
    }
}
// --8<-- [end:increase]

fn main() {
    let mut prices: [i32; 5] = [80, 120, 200, 60, 40];
    increase(&mut prices[1..3], 10);
    assert_eq!(prices[0], 80);
    assert_eq!(prices[1], 130);
    assert_eq!(prices[2], 210);
    assert_eq!(prices[3], 60);
    assert_eq!(prices[4], 40);
    assert_eq!(sum_values(&prices[..]), 520);

    increase(&mut prices[2..2], 10);
    assert_eq!(sum_values(&prices[..]), 520);
    println!(
        "五項價格={}、{}、{}、{}、{}",
        prices[0], prices[1], prices[2], prices[3], prices[4]
    );
    println!("完整總和=520，空區間沒有修改資料");
}
