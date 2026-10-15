pub mod newtypes;

use newtypes::Meters;
use newtypes::Seconds;
use newtypes::distance_value;

fn main() {
    let distance = Meters(80);
    let elapsed = Seconds(80);
    println!("路程的數字={}，單位為公尺", distance_value(&distance));
    println!("經過的數字={}，單位為秒", elapsed.0);
    println!("借完路程，原型別仍是={distance:?}");
}
