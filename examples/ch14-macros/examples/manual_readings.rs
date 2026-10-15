use ch14_macros::Reading;
use ch14_macros::read_value;

// --8<-- [start:manual-types]
#[derive(Debug, PartialEq)]
pub struct Meters(pub i32);

impl Reading for Meters {
    fn reading(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct Seconds(pub i32);

impl Reading for Seconds {
    fn reading(&self) -> i32 {
        self.0
    }
}
// --8<-- [end:manual-types]

// --8<-- [start:observe]
fn main() {
    let distance = Meters(80);
    let waiting = Seconds(30);
    println!(
        "路程數字={}，等待數字={}",
        read_value(&distance),
        read_value(&waiting)
    );
    println!("借讀後原值仍在：{distance:?}、{waiting:?}");
    assert_eq!(distance, Meters(80));
    assert_eq!(waiting, Seconds(30));
}
// --8<-- [end:observe]
