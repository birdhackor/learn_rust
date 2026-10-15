macro_rules! reading_types {
    ($($name:ident),+ $(,)?) => {
        $(struct $name(i32);)+
    };
}

reading_types!(Meters, Seconds);

fn distance_value(distance: &Meters) -> i32 {
    distance.0
}

fn main() {
    let waiting = Seconds(30);
    println!("路程={}", distance_value(&waiting));
}
