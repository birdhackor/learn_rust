use ch12_unsafe::contracts::split_i32_at_mut;

#[test]
fn safe_call_modifies_two_disjoint_regions() {
    let mut values = [80, 120, 40, 60];
    {
        let (left, right) = split_i32_at_mut(&mut values, 2).expect("2在四項數列的範圍內");
        left[0] += 20;
        right[0] += 5;
    }
    assert_eq!(values, [100, 120, 45, 60]);
}

#[test]
fn empty_storage_and_endpoints_keep_valid_views() {
    let mut values = [80, 120];
    {
        let (left, right) = split_i32_at_mut(&mut values, 0).expect("0是合法端點");
        assert_eq!(left.len(), 0);
        assert_eq!(right.len(), 2);
        assert_eq!(right[0], 80);
        assert_eq!(right[1], 120);
    }
    {
        let (left, right) = split_i32_at_mut(&mut values, 2).expect("len是合法端點");
        assert_eq!(left.len(), 2);
        assert_eq!(left[0], 80);
        assert_eq!(left[1], 120);
        assert_eq!(right.len(), 0);
    }
    let mut empty: Vec<i32> = Vec::new();
    let (left, right) = split_i32_at_mut(empty.as_mut_slice(), 0).expect("空數列仍有0端點");
    assert_eq!(left.len(), 0);
    assert_eq!(right.len(), 0);
}

#[test]
fn invalid_mid_is_rejected_without_modifying_values() {
    let mut values = [80, 120];
    match split_i32_at_mut(&mut values, 3) {
        None => {}
        Some(_) => panic!("超出長度的mid不能取得視圖"),
    }
    assert_eq!(values, [80, 120]);
}
