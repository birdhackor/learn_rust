use duplicate_a3::collection::scan;
use duplicate_a3::content::analyze;
use duplicate_a3::report::build_report;
use duplicate_a3::report::render_json;
use std::io;
use std::io::Write;
use std::path::Path;

fn main() {
    // 操作入口固定為 examples/ch15-projects/a3-complete。
    let collected =
        scan(Path::new("../materials/duplicate-demo")).expect("教材固定材料是有效根目錄");
    let report = build_report(analyze(collected)).expect("固定材料的指標與路徑可表示");
    let text = render_json(&report).expect("此模型可編碼為 JSON");
    let mut output = io::stdout();
    output
        .write_all(text.as_bytes())
        .expect("可寫出此示範的標準輸出");
    output.flush().expect("可完成此示範的緩衝輸出");
}
