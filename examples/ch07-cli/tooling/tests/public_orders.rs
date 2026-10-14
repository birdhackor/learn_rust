use ch07_tooling::pair_total;

#[test]
fn small_orders_are_charged_separately() {
    let total = pair_total(80, 120);
    assert_eq!(total, 240);
}

#[test]
fn combined_goods_do_not_make_each_order_free_shipping() {
    let total = pair_total(150, 150);
    assert_eq!(total, 340);
}
