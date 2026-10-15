use clap::Parser;
use std::path::PathBuf;

// --8<-- [start:settings]
#[derive(Debug, Parser)]
#[command(
    name = "url-checker",
    version = "0.1.0",
    about = "依原行順序報告網址的 HTTP headers 檢查結果"
)]
pub struct Settings {
    #[arg(help = "每行一個網址的 UTF-8 檔案")]
    pub filename: PathBuf,
    #[arg(long, default_value_t = 8, help = "最多同時已提交的工作數，必須大於零")]
    pub jobs: usize,
    #[arg(
        long,
        default_value_t = 10,
        help = "每筆 headers 工作的期限秒數，必須大於零"
    )]
    pub timeout_secs: u64,
    #[arg(long, help = "輸出同一份結果的 JSON 表示")]
    pub json: bool,
}
// --8<-- [end:settings]
