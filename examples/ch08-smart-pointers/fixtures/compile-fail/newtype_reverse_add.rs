use std::ops::Add;

struct Meters(i32);
struct Millimeters(i32);

impl Add for Meters {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Self;

    fn add(self, rhs: Meters) -> Self::Output {
        Self(self.0 + rhs.0 * 1000)
    }
}

fn main() {
    let joined = Meters(2) + Millimeters(500);
    println!("結果={}", joined.0);
}
