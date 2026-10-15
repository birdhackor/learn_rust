fn main() {
    // --8<-- [start:text]
    let mut first = String::new();
    first.push_str("Rust");
    let mut second = String::new();
    second.push_str("Rust");
    let view: &str = first.as_str();
    println!("owned={first:?}，借用={view:?}");
    assert_eq!(first, second);
    assert_eq!(view, second.as_str());
    println!("比較後仍可讀={}", first.as_str());
    // --8<-- [end:text]

    // --8<-- [start:references]
    let first_number: usize = 2;
    let other_number: usize = 2;
    let first_ref = &first_number;
    let other_ref = &other_number;
    assert_eq!(first_ref, other_ref);
    println!("兩份借用的內容相等={}", first_ref == other_ref);
    // --8<-- [end:references]

    // --8<-- [start:options]
    let missing: Option<usize> = None;
    let present: Option<usize> = Some(2);
    assert_eq!(missing, None);
    assert_eq!(present, Some(2));
    assert!(present != Some(3));
    assert!(present != missing);
    println!("有值={present:?}，缺值={missing:?}");

    let borrowed: Option<&usize> = Some(first_ref);
    assert_eq!(borrowed, Some(other_ref));
    println!("借到的結果={borrowed:?}");
    // --8<-- [end:options]
}
