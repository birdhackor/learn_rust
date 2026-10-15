// --8<-- [start:library-root]
pub mod pricing;
// --8<-- [end:library-root]

// --8<-- [start:unit-tests]
#[cfg(test)]
mod tests {
    use crate::pricing::subtotal;
    use crate::pricing::sum_three;
    use crate::pricing::total_with_shipping;

    #[test]
    fn small_order_adds_shipping() {
        assert_eq!(subtotal(80, 2), 160);
        assert_eq!(total_with_shipping(80, 2), 180);
    }

    #[test]
    fn shipping_threshold_includes_300() {
        assert_eq!(total_with_shipping(299, 1), 319);
        assert_eq!(total_with_shipping(300, 1), 300);
    }

    #[test]
    fn empty_order_costs_zero() {
        assert_eq!(total_with_shipping(120, 0), 0);
    }

    #[test]
    fn three_values_are_all_used() {
        assert_eq!(sum_three([120, 80, 200]), 400);
        assert_eq!(sum_three([0, -5, 5]), 0);
        assert!(subtotal(80, 2) >= 0);
    }
}
// --8<-- [end:unit-tests]
