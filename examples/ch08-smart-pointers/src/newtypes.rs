use std::fmt;
use std::ops::Add;

// --8<-- [start:aliases]
pub type MeterAlias = i32;
pub type SecondAlias = i32;

pub fn alias_distance(value: MeterAlias) -> MeterAlias {
    value
}

pub type PriceFn = Box<dyn Fn(i32) -> i32>;

pub fn price_fn(discount: bool) -> PriceFn {
    if discount {
        Box::new(|value| value - 10)
    } else {
        Box::new(|value| value + 10)
    }
}
// --8<-- [end:aliases]

// --8<-- [start:units]
#[derive(Debug, PartialEq)]
pub struct Meters(pub i32);

#[derive(Debug, PartialEq)]
pub struct Seconds(pub i32);

pub fn distance_value(distance: &Meters) -> i32 {
    distance.0
}
// --8<-- [end:units]

// --8<-- [start:add-meters]
impl Add for Meters {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

pub fn add_same<T: Add<Output = T>>(left: T, right: T) -> T {
    left + right
}
// --8<-- [end:add-meters]

// --8<-- [start:add-mixed]
#[derive(Debug, PartialEq)]
pub struct Millimeters(pub i32);

impl Add<Meters> for Millimeters {
    type Output = Self;

    fn add(self, rhs: Meters) -> Self::Output {
        Self(self.0 + rhs.0 * 1000)
    }
}
// --8<-- [end:add-mixed]

// --8<-- [start:batch]
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
// --8<-- [end:batch]

// --8<-- [start:local-trait]
pub trait BatchSize {
    fn batch_size(&self) -> usize;
}

impl BatchSize for Vec<i32> {
    fn batch_size(&self) -> usize {
        self.len()
    }
}
// --8<-- [end:local-trait]

// --8<-- [start:generic-wrapper]
pub struct Wrapped<T>(pub T);

impl<T> From<T> for Wrapped<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Wrapped<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}
// --8<-- [end:generic-wrapper]

#[cfg(test)]
mod tests {
    use crate::newtypes::MeterBatch;
    use crate::newtypes::Meters;
    use crate::newtypes::Millimeters;
    use crate::newtypes::Wrapped;
    use crate::newtypes::add_same;
    use crate::newtypes::price_fn;

    #[test]
    fn the_result_keeps_its_unit_and_converts_the_rhs_first() {
        assert_eq!(add_same(Meters(80), Meters(120)), Meters(200));
        assert_eq!(Millimeters(500) + Meters(2), Millimeters(2500));
        assert_eq!(Millimeters(0) + Meters(0), Millimeters(0));
    }

    #[test]
    fn a_batch_reports_empty_and_non_empty_routes_without_consuming_them() {
        let empty = MeterBatch::from_values(Vec::new());
        assert_eq!(empty.total(), Meters(0));
        assert_eq!(format!("{empty}").as_str(), "0 段路程，共 0 公尺");
        let route = MeterBatch::from_values(vec![Meters(80), Meters(120)]);
        assert_eq!(route.total(), Meters(200));
        assert_eq!(format!("{route}").as_str(), "2 段路程，共 200 公尺");
        assert_eq!(route.count(), 2);
    }

    #[test]
    fn alias_and_generic_wrapper_keep_the_original_behavior() {
        assert_eq!(price_fn(true)(80), 70);
        assert_eq!(price_fn(false)(80), 90);
        let mut text = String::new();
        text.push_str("Rust");
        let wrapped = Wrapped::from(text);
        let recovered = wrapped.into_inner();
        assert_eq!(recovered.as_str(), "Rust");
    }
}
