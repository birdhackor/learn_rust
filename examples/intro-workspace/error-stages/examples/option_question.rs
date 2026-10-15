// --8<-- [start:second-number]
fn second_number(values: &[i32]) -> Option<i32> {
    let value = values.get(1)?;
    Some(*value)
}
// --8<-- [end:second-number]

fn main() {
    let empty: [i32; 0] = [];
    let one = [10];
    let two = [10, 20];

    println!("零筆：{:?}", second_number(&empty[..]));
    println!("一筆：{:?}", second_number(&one[..]));
    println!("兩筆：{:?}", second_number(&two[..]));
}
