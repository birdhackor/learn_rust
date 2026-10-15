use lesson_engineering::order_subtotal;

fn main() {
    let ordinary = order_subtotal(80, 2);
    assert_eq!(ordinary, 160);
    let enabled = cfg!(feature = "batch");
    println!("保留單筆={ordinary}，batch={enabled}");

    #[cfg(feature = "batch")]
    {
        use lesson_engineering::two_orders;

        let total = two_orders([80, 0], [120, 1]);
        assert_eq!(total, 120);
        println!("第一筆數量零，兩筆合計={total}");
    }
}
