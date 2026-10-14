struct Meters(i32);
struct Seconds(i32);

fn distance_value(distance: &Meters) -> i32 {
    distance.0
}

fn main() {
    let elapsed = Seconds(80);
    println!("路程={}", distance_value(&elapsed));
}
