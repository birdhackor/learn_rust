fn main() {
    // --8<-- [start:tuple]
    let order: (i32, i32, bool) = (120, 3, true);
    let price = order.0;
    let (unit_price, count, ready) = order;
    println!("單價={price}");
    println!("拆開後：單價={unit_price}，數量={count}，備妥={ready}");
    // --8<-- [end:tuple]

    // --8<-- [start:array]
    let prices: [i32; 3] = [120, 80, 200];
    let first = prices[0];
    let third = prices[2];
    println!("第一項={first}，第三項={third}");
    // --8<-- [end:array]

    // --8<-- [start:selection]
    let length = prices.len();
    let index: usize = 1;
    let selected = prices[index];
    println!("項數={length}，選到={selected}");
    // --8<-- [end:selection]

    // --8<-- [start:repeat]
    let mut counts: [i32; 3] = [0; 3];
    counts[1] = 2;
    let middle = counts[1];
    println!("中間數量={middle}");
    // --8<-- [end:repeat]
}
