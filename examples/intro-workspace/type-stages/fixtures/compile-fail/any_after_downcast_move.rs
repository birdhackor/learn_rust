use std::any::Any;

fn main() {
    let amount: i32 = 80;
    let boxed: Box<dyn Any> = Box::new(amount);
    let attempted = boxed.downcast::<i32>();
    println!("原Box是否為i32={}", boxed.is::<i32>());
    match attempted {
        Ok(number) => println!("數字={}", *number),
        Err(_) => println!("型別不符"),
    }
}
