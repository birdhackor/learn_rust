macro_rules! reading_type {
    ($name:ident) => {
        struct $name(i32);
    };
}

reading_type!(80);

fn main() {}
