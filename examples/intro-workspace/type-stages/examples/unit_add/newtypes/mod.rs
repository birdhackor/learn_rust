use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Meters(pub i32);

#[derive(Debug, PartialEq)]
pub struct Seconds(pub i32);

pub fn distance_value(distance: &Meters) -> i32 {
    distance.0
}

impl Add for Meters {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

pub fn add_same<T: Add<Output = T>>(left: T, right: T) -> T {
    left + right
}

#[derive(Debug, PartialEq)]
pub struct Millimeters(pub i32);

impl Add<Meters> for Millimeters {
    type Output = Self;

    fn add(self, rhs: Meters) -> Self::Output {
        Self(self.0 + rhs.0 * 1000)
    }
}
