// --8<-- [start:totals]
pub fn total_now(subtotal: i32) -> i32 {
    subtotal + 20
}

pub async fn total_later(subtotal: i32) -> i32 {
    println!("async 本體：處理小計 {subtotal}");
    total_now(subtotal)
}
// --8<-- [end:totals]

// --8<-- [start:checked-total]
pub async fn checked_total(subtotal: i32) -> Result<i32, String> {
    if subtotal < 0 {
        Err("小計不能為負數".to_owned())
    } else {
        Ok(total_now(subtotal))
    }
}
// --8<-- [end:checked-total]

// --8<-- [start:lifecycle-test]
#[cfg(test)]
mod tests {
    #[tokio::test(flavor = "current_thread")]
    async fn discarding_and_awaiting_have_different_effects_on_the_body() {
        let mut visits = 0;
        let unused_work = async {
            visits += 1;
            visits
        };
        drop(unused_work);
        assert_eq!(visits, 0);

        let used_work = async {
            visits += 1;
            visits
        };
        let result = used_work.await;
        assert_eq!(result, 1);
        assert_eq!(visits, 1);
    }
}
// --8<-- [end:lifecycle-test]
