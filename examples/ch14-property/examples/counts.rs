use ch14_property::count_of;
use ch14_property::frequencies;
use ch14_property::total_count;

fn main() {
    let values = [0, 2, 0, -3];
    let counts = frequencies(&values[..]);
    for key in [0, 2, -3, 1] {
        println!("鍵{key}的次數={}", count_of(&counts, key));
    }
    println!(
        "總次數={}，原輸入長度={}",
        total_count(&counts),
        values.len()
    );
}
