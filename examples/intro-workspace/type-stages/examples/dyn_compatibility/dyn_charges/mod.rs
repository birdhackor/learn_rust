pub struct Bundle {
    pub price: i32,
}

pub trait Charge {
    fn amount(&self) -> i32;
}

impl Charge for Bundle {
    fn amount(&self) -> i32 {
        self.price
    }
}

// --8<-- [start:rebuild]
pub trait RebuildCharge: Charge {
    fn rebuild(&self) -> Self
    where
        Self: Sized;
}

impl RebuildCharge for Bundle {
    fn rebuild(&self) -> Self {
        Bundle { price: self.price }
    }
}
// --8<-- [end:rebuild]
// --8<-- [start:measure]
pub trait Measure {
    type Output;

    fn measure(&self) -> Self::Output;
}

impl Measure for Bundle {
    type Output = i32;

    fn measure(&self) -> Self::Output {
        self.price
    }
}
// --8<-- [end:measure]
