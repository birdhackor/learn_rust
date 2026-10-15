use intro_modules_basic::pricing::total_with_shipping;

// --8<-- [start:documented-function]
/// 計算兩筆各買一件商品的訂單總額，各自套用運費規則。
///
/// 單價使用非負的小整數，計算結果不得超過 `i32` 範圍。
///
/// # Examples
///
/// ```rust
/// use intro_cargo_docs_answer::pair_total;
///
/// let total = pair_total(80, 120);
/// assert_eq!(total, 240);
/// ```
///
/// 一筆免運、另一筆仍須付運費：
///
/// ```rust
/// use intro_cargo_docs_answer::pair_total;
///
/// let total = pair_total(300, 120);
/// assert_eq!(total, 440);
/// ```
pub fn pair_total(first_price: i32, second_price: i32) -> i32 {
    total_with_shipping(first_price, 1) + total_with_shipping(second_price, 1)
}
// --8<-- [end:documented-function]

// --8<-- [start:unit-tests]
#[cfg(test)]
mod tests {
    use crate::pair_total;

    #[test]
    fn each_order_keeps_its_own_shipping_threshold() {
        assert_eq!(pair_total(300, 120), 440);
    }
}
// --8<-- [end:unit-tests]
