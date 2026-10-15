use intro_control::pricing::subtotal;
use intro_control::pricing::total_with_shipping;

fn main() {
    let goods = subtotal(140, 2);
    let total = total_with_shipping(140, 2);
    println!("商品小計={goods}，含運總額={total}");
}

#[cfg(test)]
mod tests {
    use intro_control::pricing::subtotal;
    use intro_control::pricing::total_with_shipping;

    #[test]
    fn shipping_uses_goods_subtotal() {
        // --8<-- [start:answer-checks]
        let goods = subtotal(140, 2);
        let total = total_with_shipping(140, 2);
        assert_eq!(goods, 280);
        assert_eq!(total, 300);
        // --8<-- [end:answer-checks]
    }
}
