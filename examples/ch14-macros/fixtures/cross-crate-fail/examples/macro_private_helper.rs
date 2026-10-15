use ch14_macros::private_reading;
use ch14_macros::reading_type;

reading_type!(Meters);

fn main() {
    let distance = Meters(80);
    println!("讀值={}", private_reading!(&distance));
}
