use ch05_errors::propagation::second_number;

fn main() {
    let empty: [i32; 0] = [];
    let one = [10];
    let two = [10, 20];

    println!("零筆：{:?}", second_number(&empty[..]));
    println!("一筆：{:?}", second_number(&one[..]));
    println!("兩筆：{:?}", second_number(&two[..]));
}
