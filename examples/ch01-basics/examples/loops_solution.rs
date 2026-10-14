fn sum_three(values: [i32; 3]) -> i32 {
    let mut total = 0;
    for value in values {
        total += value;
    }
    total
}

fn main() {
    let first = sum_three([120, 80, 200]);
    let second = sum_three([0, -5, 5]);
    println!("第一組={first}，第二組={second}");
}
