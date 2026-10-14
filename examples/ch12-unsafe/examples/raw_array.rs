fn main() {
    let mut values = [80, 120, 40];
    let view: &mut [i32] = &mut values;
    let len = view.len();
    let start = view.as_mut_ptr();

    // SAFETY: start衍生自整個三項i32 array的mutable slice，array仍存活，
    // 元素已初始化且正確aligned。1在有效索引內，len形成同一allocation的
    // one-past端點；這份array的byte範圍不超過isize::MAX且不會地址overflow。
    // raw操作期間不從view或其他reference讀寫；end只檢查，不dereference。
    let (before, after, end_is_null) = unsafe {
        let second = start.add(1);
        let end = start.add(len);
        let before = *second;
        *second = 140;
        (before, *second, end.is_null())
    };

    println!("第二項由{before}改成{after}");
    println!("one-past pointer是null={end_is_null}");
    println!("原array總額={}", values[0] + values[1] + values[2]);
    assert_eq!(values, [80, 140, 40]);
}
