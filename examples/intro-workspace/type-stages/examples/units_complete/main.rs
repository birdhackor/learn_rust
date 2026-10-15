pub mod newtypes;

use newtypes::Seconds;
use std::fmt;

struct TimeBatch(Vec<Seconds>);

impl TimeBatch {
    fn from_values(values: Vec<Seconds>) -> Self {
        Self(values)
    }

    fn total(&self) -> Seconds {
        let values = self.0.as_slice();
        let mut index = 0;
        let mut total = 0;
        while index < values.len() {
            total += values[index].0;
            index += 1;
        }
        Seconds(total)
    }
}

impl fmt::Display for TimeBatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} 段等待，共 {} 秒", self.0.len(), self.total().0)
    }
}

fn main() {
    let waiting = TimeBatch::from_values(vec![Seconds(30), Seconds(60)]);
    println!("{waiting}");
    assert_eq!(waiting.total(), Seconds(90));
}
