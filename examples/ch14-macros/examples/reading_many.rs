use ch14_macros::Reading;
use ch14_macros::read_value;

// --8<-- [start:repeat-rule]
macro_rules! reading_types {
    ($($name:ident),+ $(,)?) => {
        $(
            #[derive(Debug, PartialEq)]
            pub struct $name(pub i32);

            impl Reading for $name {
                fn reading(&self) -> i32 {
                    self.0
                }
            }
        )+
    };
}
// --8<-- [end:repeat-rule]

// --8<-- [start:repeat-call]
reading_types!(Meters, Seconds, Millimeters,);
// --8<-- [end:repeat-call]

// --8<-- [start:observe]
fn main() {
    let distance = Meters(80);
    let waiting = Seconds(30);
    let small_distance = Millimeters(500);
    println!(
        "路程數字={}，等待數字={}",
        read_value(&distance),
        read_value(&waiting)
    );
    println!("毫米數字={}", read_value(&small_distance));
    assert_eq!(distance, Meters(80));
    assert_eq!(waiting, Seconds(30));
    assert_eq!(small_distance, Millimeters(500));
}
// --8<-- [end:observe]

#[cfg(test)]
mod tests {
    use crate::Meters;
    use crate::Millimeters;
    use crate::Seconds;
    use ch14_macros::read_value;

    #[test]
    fn generated_types_can_share_the_reader_without_consuming_the_sources() {
        let distance = Meters(0);
        let waiting = Seconds(30);
        let small_distance = Millimeters(-5);
        assert_eq!(read_value(&distance), 0);
        assert_eq!(read_value(&waiting), 30);
        assert_eq!(read_value(&small_distance), -5);
        assert_eq!(distance, Meters(0));
        assert_eq!(waiting, Seconds(30));
        assert_eq!(small_distance, Millimeters(-5));
    }
}
