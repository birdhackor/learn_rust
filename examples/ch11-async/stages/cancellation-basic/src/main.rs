use ch11_cancellation_basic::stop_demo;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let batch = stop_demo().await;
    println!("結束原因：{:?}", batch.reason);
    println!(
        "共提交 {}；追蹤峰值 {}",
        batch.submitted, batch.peak_tracked
    );
    for report in batch.reports.as_slice() {
        println!("input {}：{:?}", report.input, report.outcome);
    }
    println!("已回收全部提交工作，並清理未提交輸入");
}
