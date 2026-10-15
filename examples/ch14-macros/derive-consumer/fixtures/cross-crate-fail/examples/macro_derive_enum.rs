use ch14_reading_derive::ReadingValue;

#[derive(ReadingValue)]
enum ReadingState {
    Ready(i32),
    Empty,
}

fn main() {}
