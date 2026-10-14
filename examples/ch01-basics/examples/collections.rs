fn main() {
    let order: (i32, i32) = (120, 3);
    let (price, count) = order;
    let total = price * count;
    println!("單價={}，數量={}，總額={total}", order.0, order.1);

    let prices: [i32; 3] = [120, 80, 200];
    let position: usize = 1;
    let length = prices.len();
    let selected = prices[position];
    let [first, second, third] = prices;
    println!("長度={length}，索引1的值={selected}");
    println!("三筆價格={first}、{second}、{third}");

    let zeroes: [i32; 3] = [0; 3];
    let first_zero = zeroes[0];
    println!("重複建立的第一項={first_zero}");
}
