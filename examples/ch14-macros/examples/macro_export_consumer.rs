use ch14_macros::read_value;
use ch14_macros::reading_type;

// --8<-- [start:own-trait]
pub trait Reading {
    fn unrelated(&self);
}
// --8<-- [end:own-trait]

// --8<-- [start:export-calls]
reading_type!(Meters);
reading_type!(Seconds);

fn main() {
    let distance = Meters(80);
    let waiting = Seconds(30);
    println!(
        "library的Reading仍取得={}、{}",
        read_value(&distance),
        read_value(&waiting)
    );
    assert_eq!(distance, Meters(80));
    assert_eq!(waiting, Seconds(30));
}
// --8<-- [end:export-calls]
