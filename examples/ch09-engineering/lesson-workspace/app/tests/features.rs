use lesson_engineering::order_subtotal;

#[test]
fn ordinary_order_works_in_every_feature_set() {
    assert_eq!(order_subtotal(80, 2), 160);
}

#[cfg(feature = "batch")]
#[test]
fn batch_uses_each_orders_price_and_count() {
    use lesson_engineering::two_orders;

    assert_eq!(two_orders([80, 2], [120, 1]), 280);
    assert_eq!(two_orders([80, 0], [120, 1]), 120);
}
