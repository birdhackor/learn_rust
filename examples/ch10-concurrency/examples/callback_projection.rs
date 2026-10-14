use ch10_concurrency::higher_ranked::sum_selected;

fn second(pair: &(i32, i32)) -> &i32 {
    &pair.1
}

fn main() {
    let choose: fn(&(i32, i32)) -> &i32 = second;
    let result = sum_selected(&choose);
    assert_eq!(result, 126);
    println!("兩次各借出第二個欄位，合計={result}");
}
