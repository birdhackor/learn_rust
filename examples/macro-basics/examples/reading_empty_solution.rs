use macro_basics::Reading;
use macro_basics::read_value;

macro_rules! reading_types {
    ($($name:ident),* $(,)?) => {
        $(
            #[derive(Debug, PartialEq)]
            pub struct $name(pub i32);

            impl Reading for $name {
                fn reading(&self) -> i32 {
                    self.0
                }
            }
        )*
    };
}

reading_types!();
reading_types!(Meters, Seconds);

fn main() {
    let distance = Meters(80);
    let waiting = Seconds(30);
    assert_eq!(read_value(&distance), 80);
    assert_eq!(read_value(&waiting), 30);
    println!("空呼叫沒有新增型別；原路程與等待仍是80、30");
}
