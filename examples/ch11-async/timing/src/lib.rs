use ch11_foundations::checked_total;
use std::time::Duration;
use tokio::time::sleep;

// --8<-- [start:total-after]
pub async fn total_after(subtotal: i32, delay: Duration) -> Result<i32, String> {
    let total = checked_total(subtotal).await?;
    sleep(delay).await;
    Ok(total)
}
// --8<-- [end:total-after]

// --8<-- [start:two-steps]
pub async fn two_steps(completed: &mut usize, delay: Duration) {
    sleep(delay).await;
    *completed += 1;
    sleep(delay).await;
    *completed += 1;
}
// --8<-- [end:two-steps]
