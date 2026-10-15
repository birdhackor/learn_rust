use reqwest::Client;
use reqwest::redirect::Policy;
use std::time::Duration;
use url::Url;

// --8<-- [start:models]
#[derive(Debug)]
pub enum InputIssue {
    Url(url::ParseError),
    Scheme(String),
}

#[derive(Debug)]
pub enum Outcome {
    Http {
        final_url: String,
        status: u16,
        passed: bool,
    },
    Input(InputIssue),
    Request(reqwest::Error),
    Timeout,
}

#[derive(Debug)]
pub struct Check {
    pub original: String,
    pub elapsed_ms: Option<u128>,
    pub outcome: Outcome,
}
// --8<-- [end:models]

// --8<-- [start:helpers]
pub fn default_limit() -> Duration {
    Duration::from_secs(10)
}

pub fn build_client(direct: bool) -> Result<Client, reqwest::Error> {
    let mut builder = Client::builder().redirect(Policy::limited(5));
    if direct {
        builder = builder.no_proxy();
    }
    builder.build()
}

pub fn parse_http_url(input: &str) -> Result<Url, InputIssue> {
    let url = match Url::parse(input) {
        Ok(url) => url,
        Err(error) => return Err(InputIssue::Url(error)),
    };
    match url.scheme() {
        "http" | "https" => Ok(url),
        other => Err(InputIssue::Scheme(other.to_owned())),
    }
}
// --8<-- [end:helpers]

// --8<-- [start:stub]
pub async fn check(_client: &Client, original: String, _limit: Duration) -> Check {
    // 尚未實作，不表示輸入真的不合法；請替換整份 body。
    Check {
        original,
        elapsed_ms: None,
        outcome: Outcome::Input(InputIssue::Scheme("B1 尚未實作".to_owned())),
    }
}
// --8<-- [end:stub]
