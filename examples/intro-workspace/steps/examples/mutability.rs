fn main() {
    // --8<-- [start:mutable]
    let mut total: i32 = 360;
    println!("加運費前={total}");
    total += 20;
    println!("加運費後={total}");
    // --8<-- [end:mutable]

    // --8<-- [start:shadow]
    let count: i32 = 3;
    let count = count + 1;
    println!("下一階段數量={count}");
    let count: f64 = 4.0;
    println!("新型別的數量={count}");
    // --8<-- [end:shadow]
}
