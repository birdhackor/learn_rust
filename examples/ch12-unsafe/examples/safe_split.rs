use ch12_unsafe::contracts::split_i32_at_mut;

fn main() {
    let mut values = [80, 120, 40, 60];
    {
        let (left, right) = split_i32_at_mut(&mut values, 2).expect("2在四項數列的範圍內");
        println!(
            "兩段分別有{}項與{}項，各自第一項={}、{}",
            left.len(),
            right.len(),
            left[0],
            right[0]
        );
        left[0] += 20;
        right[0] += 5;
    }
    assert_eq!(values, [100, 120, 45, 60]);
    println!("借用結束後原數列={values:?}");

    match split_i32_at_mut(&mut values, 5) {
        None => println!("mid超過長度，取得None；原數列未變"),
        Some(_) => panic!("超過長度的mid不應取得視圖"),
    }
    assert_eq!(values, [100, 120, 45, 60]);
}
