// --8<-- [start:range-function]
fn range_sum(start: i32, end: i32) -> i32 {
    let mut total = 0;
    for number in start..end {
        total += number;
    }
    total
}
// --8<-- [end:range-function]

fn main() {
    // --8<-- [start:array-loop]
    let prices: [i32; 3] = [120, 80, 200];
    let mut total = 0;
    for price in prices {
        total += price;
    }
    println!("價目總和={total}");
    // --8<-- [end:array-loop]

    let excluded = range_sum(0, 3);
    let empty = range_sum(3, 3);
    // --8<-- [start:inclusive-loop]
    let mut included = 0;
    for number in 0..=3 {
        included += number;
    }
    // --8<-- [end:inclusive-loop]
    println!("不含終點的總和={excluded}");
    println!("包含終點的總和={included}");
    println!("起終點相等的半開範圍總和={empty}");
}
