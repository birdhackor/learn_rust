// --8<-- [start:sum]
fn sum_values(values: &[i32]) -> i32 {
    let mut total = 0;
    for value in values {
        total += *value;
    }
    total
}
// --8<-- [end:sum]

fn main() {
    // --8<-- [start:selection]
    let prices: [i32; 5] = [80, 120, 200, 60, 40];
    let selected: &[i32] = &prices[1..4];
    println!("選到的長度={}，第一項={}", selected.len(), selected[0]);
    println!("選到的總和={}", sum_values(selected));
    // --8<-- [end:selection]
    println!("全部總和={}", sum_values(&prices[..]));
    println!("空區間總和={}", sum_values(&prices[2..2]));

    let short: [i32; 2] = [80, 120];
    println!("兩項資料的總和={}", sum_values(&short[..]));
    assert!(selected.len() == 3);
    assert_eq!(selected[0], 120);
    assert_eq!(sum_values(selected), 380);
    assert_eq!(sum_values(&prices[..]), 500);
    assert_eq!(sum_values(&prices[2..2]), 0);
}
