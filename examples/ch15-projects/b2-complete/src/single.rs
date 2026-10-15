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

// --8<-- [start:solution]
// --8<-- [start:input-stage]
pub async fn check(client: &Client, original: String, limit: Duration) -> Check {
    let url = match parse_http_url(original.as_str()) {
        Ok(url) => url,
        Err(issue) => {
            return Check {
                original,
                elapsed_ms: None,
                outcome: Outcome::Input(issue),
            };
        }
    };
    // --8<-- [end:input-stage]
    // --8<-- [start:headers-stage]
    let start = std::time::Instant::now();
    let work = async { client.get(url).send().await };
    let result = tokio::time::timeout(limit, work).await;
    let elapsed_ms = Some(start.elapsed().as_millis());
    let outcome = match result {
        Ok(Ok(response)) => {
            let status = response.status();
            let final_url = response.url().as_str().to_owned();
            drop(response);
            Outcome::Http {
                final_url,
                status: status.as_u16(),
                passed: status.is_success(),
            }
        }
        Ok(Err(error)) => {
            if error.is_timeout() {
                Outcome::Timeout
            } else {
                Outcome::Request(error)
            }
        }
        Err(_) => Outcome::Timeout,
    };
    Check {
        original,
        elapsed_ms,
        outcome,
    }
}
// --8<-- [end:headers-stage]
// --8<-- [end:solution]
