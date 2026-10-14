fn price_at(position: usize) -> i32 {
    let prices = [120, 80, 200];
    prices[position]
}

fn main() {
    let selected = price_at(3);
    println!("{selected}");
}
