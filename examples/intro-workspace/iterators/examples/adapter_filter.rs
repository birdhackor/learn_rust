fn main() {
    let values = vec![-2, 0, 3, 3];
    let filtered = values.as_slice().iter().filter(|item: &&i32| **item >= 0);
    for item in filtered {
        println!("保留的原數字={}", *item);
    }

    let texts = vec!["Rust".to_owned(), "hi".to_owned()];
    let kept = texts
        .into_iter()
        .filter(|item: &String| item.as_str().len() >= 4);
    for item in kept {
        println!("保留的 owned 文字={}", item.as_str());
    }
}
