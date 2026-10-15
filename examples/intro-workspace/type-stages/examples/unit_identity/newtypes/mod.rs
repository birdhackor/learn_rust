#[derive(Debug, PartialEq)]
pub struct Meters(pub i32);

#[derive(Debug, PartialEq)]
pub struct Seconds(pub i32);

pub fn distance_value(distance: &Meters) -> i32 {
    distance.0
}
