use ch11_http::Inspection;

fn status_line(inspection: &Inspection) -> String {
    let label = if inspection.passed {
        "通過"
    } else {
        "HTTP 未通過"
    };
    format!(
        "{label}：{}（HTTP {}）",
        inspection.final_url, inspection.status
    )
}

fn main() {
    let mut inspection = Inspection {
        final_url: "https://example.org/notes".to_owned(),
        status: 200,
        passed: true,
        body: None,
    };
    let summary = status_line(&inspection);
    assert_eq!(
        summary.as_str(),
        "通過：https://example.org/notes（HTTP 200）"
    );
    inspection.status = 404;
    inspection.passed = false;
    let summary = status_line(&inspection);
    assert_eq!(
        summary.as_str(),
        "HTTP 未通過：https://example.org/notes（HTTP 404）"
    );
    println!("{}", status_line(&inspection));
}
