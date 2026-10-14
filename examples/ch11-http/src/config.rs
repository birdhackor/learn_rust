use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "http-inspect",
    version = "0.1.0",
    about = "檢查一個 HTTP(S) URL"
)]
pub struct Settings {
    #[arg(help = "包含 http:// 或 https:// 的 URL")]
    pub url: String,
    #[arg(long, help = "另外讀完整 body，並轉成文字")]
    pub body: bool,
    #[arg(long, help = "直接連線，不使用 proxy")]
    pub direct: bool,
}
