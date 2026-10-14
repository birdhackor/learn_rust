use ch10_concurrency::higher_ranked::with_local;

fn main() {
    let result = with_local(|number: &i32| *number + 120);
    assert_eq!(result, 200);
    println!("函式內提供借用，結果={result}");
}
