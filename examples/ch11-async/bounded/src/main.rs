use ch11_bounded::run_demo;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let batch = run_demo().await;
    println!(
        "上限 2；追蹤峰值 {}；共提交 {}",
        batch.peak_tracked, batch.submitted
    );
    for report in batch.reports.as_slice() {
        println!("input {}：{:?}", report.input, report.outcome);
    }
    println!("五份工作全部回收；沒有未提交輸入");
}
