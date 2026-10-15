use ch14_reading_derive::ReadingValue;

#[derive(ReadingValue)]
struct Holder<T>(T);

fn main() {}
