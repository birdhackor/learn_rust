use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Meters(pub i32);

#[derive(Debug, PartialEq)]
pub struct Seconds(pub i32);

pub fn distance_value(distance: &Meters) -> i32 {
    distance.0
}

pub struct MeterBatch(Vec<Meters>);

impl MeterBatch {
    pub fn from_values(values: Vec<Meters>) -> Self {
        Self(values)
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn total(&self) -> Meters {
        let values = self.0.as_slice();
        let mut index = 0;
        let mut total = 0;
        while index < values.len() {
            total += values[index].0;
            index += 1;
        }
        Meters(total)
    }
}

impl fmt::Display for MeterBatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} 段路程，共 {} 公尺", self.count(), self.total().0)
    }
}

pub trait BatchSize {
    fn batch_size(&self) -> usize;
}

impl BatchSize for Vec<i32> {
    fn batch_size(&self) -> usize {
        self.len()
    }
}
