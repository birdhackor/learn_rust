pub mod payloads;

use payloads::concrete_id;
use std::any::Any;
use std::any::TypeId;

fn main() {
    let amount: i32 = 80;
    let boxed: Box<dyn Any> = Box::new(amount);
    let view: &dyn Any = &*boxed;
    println!(
        "內部資料的TypeId是i32={}",
        view.type_id() == TypeId::of::<i32>()
    );
    println!(
        "把Box本身當T時得到Box的TypeId={}",
        concrete_id(&boxed) == TypeId::of::<Box<dyn Any>>()
    );
}
