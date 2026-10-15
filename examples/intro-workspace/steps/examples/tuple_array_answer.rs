fn main() {
    let order: (i32, i32, bool) = (80, 2, true);
    let (price, count, ready) = order;
    let total = price * count;
    println!("訂單總額={total}，備妥={ready}");

    let prices: [i32; 3] = [120, 80, 200];
    let index: usize = 2;
    let selected = prices[index];
    let length = prices.len();
    println!("索引={index}，價格={selected}，項數={length}");
}
