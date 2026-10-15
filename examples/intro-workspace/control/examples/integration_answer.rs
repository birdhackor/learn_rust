use intro_control::pricing::subtotal;
use intro_control::pricing::sum_three;
use intro_control::pricing::total_with_shipping;

fn main() {
    let goods = subtotal(140, 2);
    let total = total_with_shipping(140, 2);
    let sum = sum_three([120, 80, 200]);
    println!("商品小計={goods}，含運總額={total}");
    println!("價目總和={sum}");
}

#[cfg(test)]
mod tests {
    use intro_control::pricing::subtotal;
    use intro_control::pricing::sum_three;
    use intro_control::pricing::total_with_shipping;

    #[test]
    fn goods_threshold_and_empty_order() {
        assert_eq!(subtotal(140, 2), 280);
        assert_eq!(total_with_shipping(140, 2), 300);
        assert_eq!(total_with_shipping(300, 1), 300);
        assert_eq!(total_with_shipping(120, 0), 0);
    }

    #[test]
    fn both_arrays_are_used() {
        assert_eq!(sum_three([120, 80, 200]), 400);
        assert_eq!(sum_three([0, -5, 5]), 0);
    }
}
