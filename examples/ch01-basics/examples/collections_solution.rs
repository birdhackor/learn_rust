fn main() {
    let order: (i32, i32) = (80, 3);
    let (price, count) = order;
    let subtotal = price * count;
    let prices: [i32; 3] = [80, 120, 200];
    let position: usize = 2;
    let selected = prices[position];
    println!("小計={subtotal}，最後一項={selected}");
}
