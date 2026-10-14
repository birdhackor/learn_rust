use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "note-search",
    version = "0.1.0",
    about = "搜尋 UTF-8 文字檔中的整行"
)]
pub struct Settings {
    #[arg(help = "非空的搜尋文字")]
    pub query: String,
    #[arg(help = "要讀取的文字檔")]
    pub filename: PathBuf,
    #[arg(long, help = "忽略 ASCII 字母的大小寫")]
    pub ignore_case: bool,
    #[arg(long, help = "輸出 JSON 報告")]
    pub json: bool,
}
