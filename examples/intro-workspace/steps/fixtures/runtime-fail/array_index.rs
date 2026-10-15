fn selected_price(index: usize) -> i32 {
    let prices: [i32; 3] = [120, 80, 200];
    prices[index]
}

fn main() {
    let price = selected_price(3);
    println!("價格={price}");
}
