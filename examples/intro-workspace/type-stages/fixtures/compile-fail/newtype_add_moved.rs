use std::ops::Add;

struct Meters(i32);

impl Add for Meters {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

fn main() {
    let left = Meters(80);
    let joined = left + Meters(120);
    println!("結果={}，原左邊={}", joined.0, left.0);
}
