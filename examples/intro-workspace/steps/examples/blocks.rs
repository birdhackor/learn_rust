fn main() {
    let price: i32 = 120;
    let count: i32 = 3;
    // --8<-- [start:block]
    let total = {
        let item_total = price * count;
        item_total + 20
    };
    // --8<-- [end:block]
    println!("含運總額={total}");

    // --8<-- [start:nested_shadow]
    let count = 3;
    {
        let count = count + 1;
        println!("內層數量={count}");
    }
    println!("外層數量={count}");
    // --8<-- [end:nested_shadow]
}
