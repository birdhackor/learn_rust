macro_rules! reading_types {
    ($($name:ident),+ $(,)?) => {
        $(struct $name(i32);)+
    };
}

reading_types!();

fn main() {}
