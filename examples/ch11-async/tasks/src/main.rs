use ch11_tasks::run_demo;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let reports = run_demo().await;
    for report in reports.as_slice() {
        println!("input {}：{:?}", report.input, report.outcome);
    }
    println!("四份 task 都已回收");
}
