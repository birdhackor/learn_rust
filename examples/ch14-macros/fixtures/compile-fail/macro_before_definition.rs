reading_type!(Meters);

macro_rules! reading_type {
    ($name:ident) => {
        struct $name(i32);
    };
}

fn main() {}
