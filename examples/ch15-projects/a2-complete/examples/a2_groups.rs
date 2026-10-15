use duplicate_a2::collection::scan;
use duplicate_a2::content::analyze;
use std::path::Path;

fn main() {
    // 操作入口固定為 examples/ch15-projects/a2-complete。
    let report = scan(Path::new("../materials/duplicate-demo")).expect("教材固定材料是有效根目錄");
    let result = analyze(report);
    println!(
        "A2：{} 個重複群組，{} 個收集問題，{} 個內容問題",
        result.groups.len(),
        result.scan_issues.len(),
        result.file_issues.len()
    );
    for group in result.groups {
        println!("{} bytes，{} 個路徑", group.len, group.paths.len());
        for path in group.paths {
            println!("  {}", path.display());
        }
    }
    for issue in result.scan_issues {
        eprintln!("{} {:?}：{}", issue.action, issue.path, issue.cause);
    }
    for issue in result.file_issues {
        eprintln!("{issue}");
    }
}
