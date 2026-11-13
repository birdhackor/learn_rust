use std::convert::Infallible;

fn convert<T: TryFrom<i32>>(value: i32) -> Result<T, T::Error> {
    T::try_from(value)
}

fn main() {
    let result: Result<i64, !> = convert(3);
    let Ok(value) = result;
    println!("沒有Err分支的轉換={value}");

    let compatible: Result<i64, Infallible> = convert(-1);
    let Ok(negative) = compatible;
    println!("Infallible別名的轉換={negative}");
}
