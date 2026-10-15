fn main() {
    let mut values = [10, 20, 30];
    let view: &mut [i32] = &mut values;
    let start = view.as_mut_ptr();

    // SAFETY: array在本次raw存取期間不移動且仍存活，三個i32已初始化。
    // start來自涵蓋整個array的mutable slice，add(2)仍在同一allocation，
    // 第三項正確aligned。本例不從其他reference存取，也沒有其他thread。
    let before = unsafe {
        let third = start.add(2);
        let before = *third;
        *third = 50;
        before
    };

    println!("第三項原值={before}，修改後={}", values[2]);
    assert_eq!(values, [10, 20, 50]);
    assert_eq!(values[0] + values[1] + values[2], 80);
}
