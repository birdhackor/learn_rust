use ch14_macros::read_value;
use ch14_reading_consumer::Meters;
use ch14_reading_derive::ReadingValue;

type RawReading = i32;

#[derive(Debug, PartialEq, ReadingValue)]
struct Millimeters(RawReading);

fn main() {
    let distance = Meters(80);
    let detail = Millimeters(-5);
    let values = (read_value(&distance), read_value(&detail));
    assert_eq!(values, (80, -5));
    assert_eq!(distance, Meters(80));
    assert_eq!(detail, Millimeters(-5));
    println!("原共用函式借讀={values:?}；原值={detail:?}");
}
