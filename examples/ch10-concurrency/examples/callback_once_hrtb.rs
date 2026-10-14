use ch10_concurrency::higher_ranked::with_local;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let take = move |number: &i32| {
        drop(text);
        *number + 120
    };

    let result = with_local(take);
    assert_eq!(result, 200);
    println!("工作接手owned文字，呼叫一次，結果={result}");
}
