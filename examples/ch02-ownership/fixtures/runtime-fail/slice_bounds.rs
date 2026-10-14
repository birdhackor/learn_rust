fn selected_value(values: &[i32], position: usize) -> i32 {
    values[position]
}

fn main() {
    let prices: [i32; 5] = [80, 120, 200, 60, 40];
    let selected = &prices[1..4];
    println!("選取值={}", selected_value(selected, 3));
}
