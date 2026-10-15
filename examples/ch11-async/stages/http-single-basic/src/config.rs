use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "async-http-inspect",
    version = "0.1.0",
    about = "以單一 async 工作檢查 HTTP(S) URL"
)]
pub struct Settings {
    #[arg(help = "包含 http:// 或 https:// 的 URL")]
    pub url: String,
    #[arg(long, help = "另外等待完整 body，並轉成文字")]
    pub body: bool,
    #[arg(long, help = "直接連線，不使用 proxy")]
    pub direct: bool,
}
