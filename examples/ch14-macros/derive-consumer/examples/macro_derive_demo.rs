use ch14_macros::read_value;
use ch14_reading_consumer::Meters;
use ch14_reading_consumer::Seconds;

// --8<-- [start:consumer-main]
fn main() {
    let distance = Meters(80);
    let waiting = Seconds(30);
    println!(
        "derive後仍借讀={}、{}",
        read_value(&distance),
        read_value(&waiting)
    );
    assert_eq!(distance, Meters(80));
    assert_eq!(waiting, Seconds(30));
}
// --8<-- [end:consumer-main]
