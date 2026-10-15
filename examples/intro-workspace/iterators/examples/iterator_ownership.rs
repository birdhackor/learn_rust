fn main() {
    let mut first = String::new();
    first.push_str("Rust");
    let mut second = String::new();
    second.push_str("Python");
    let mut texts = vec![first, second];

    for item in texts.as_slice().iter() {
        println!("shared item={}", item.as_str());
    }

    let ending: &str = "!";
    for item in texts.as_mut_slice().iter_mut() {
        item.push_str(ending);
    }
    println!("mutable 走訪後，原第一項={}", texts.as_slice()[0].as_str());

    let mut saved = Vec::new();
    for mut item in texts.into_iter() {
        item.push_str(" saved");
        saved.push(item);
    }
    println!("owned 第一項={}", saved.as_slice()[0].as_str());
    println!("owned 第二項={}", saved.as_slice()[1].as_str());
    assert_eq!(saved.len(), 2);
    assert_eq!(saved.as_slice()[0].as_str(), "Rust! saved");
}
