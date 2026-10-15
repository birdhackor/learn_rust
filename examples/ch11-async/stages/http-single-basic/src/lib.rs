// --8<-- [start:imports]
pub mod config;
pub mod error;

use crate::config::Settings;
use crate::error::AppError;
use reqwest::Client;
use reqwest::redirect::Policy;
use std::io;
use std::io::Write;
use std::time::Duration;
use std::time::Instant;
use url::Url;
// --8<-- [end:imports]

// --8<-- [start:models]
#[derive(Debug, PartialEq)]
pub struct Inspection {
    pub final_url: String,
    pub status: u16,
    pub passed: bool,
    pub body: Option<String>,
    pub elapsed_ms: u128,
}
// --8<-- [end:models]

// --8<-- [start:input-client]
pub fn parse_http_url(input: &str) -> Result<Url, AppError> {
    let url = match Url::parse(input) {
        Ok(url) => url,
        Err(error) => return Err(AppError::Url(error)),
    };
    match url.scheme() {
        "http" | "https" => Ok(url),
        other => Err(AppError::Scheme(other.to_owned())),
    }
}

pub fn build_client(direct: bool, limit: Duration) -> Result<Client, AppError> {
    let mut builder = Client::builder()
        .redirect(Policy::limited(5))
        .timeout(limit);
    if direct {
        builder = builder.no_proxy();
    }
    match builder.build() {
        Ok(client) => Ok(client),
        Err(error) => Err(AppError::Client(error)),
    }
}
// --8<-- [end:input-client]

// --8<-- [start:inspect]
pub async fn inspect(client: &Client, url: Url, read_body: bool) -> Result<Inspection, AppError> {
    let start = Instant::now();
    let response = match client.get(url).send().await {
        Ok(response) => response,
        Err(error) => return Err(AppError::Request(error)),
    };
    let status = response.status();
    let final_url = response.url().as_str().to_owned();
    let passed = status.is_success();
    let body = if read_body {
        match response.text().await {
            Ok(text) => Some(text),
            Err(error) => {
                return Err(AppError::Body {
                    status: status.as_u16(),
                    error,
                });
            }
        }
    } else {
        None
    };
    Ok(Inspection {
        final_url,
        status: status.as_u16(),
        passed,
        body,
        elapsed_ms: start.elapsed().as_millis(),
    })
}
// --8<-- [end:inspect]

// --8<-- [start:render-run]
pub fn render<W: Write>(inspection: &Inspection, output: &mut W) -> io::Result<()> {
    writeln!(output, "final URL: {}", inspection.final_url)?;
    writeln!(output, "HTTP status: {}", inspection.status)?;
    writeln!(output, "passed: {}", inspection.passed)?;
    writeln!(output, "elapsed: {} ms", inspection.elapsed_ms)?;
    if let Some(body) = &inspection.body {
        writeln!(output, "body:\n{body}")?;
    }
    output.flush()
}

pub async fn run<W: Write>(settings: &Settings, output: &mut W) -> Result<bool, AppError> {
    let url = parse_http_url(settings.url.as_str())?;
    let client = build_client(settings.direct, Duration::from_secs(10))?;
    let inspection = inspect(&client, url, settings.body).await?;
    match render(&inspection, output) {
        Ok(()) => Ok(inspection.passed),
        Err(error) => Err(AppError::Output(error)),
    }
}
// --8<-- [end:render-run]

#[cfg(test)]
mod tests {
    use crate::Inspection;
    use crate::config::Settings;
    use crate::error::AppError;
    use crate::render;
    use crate::run;
    use std::io;
    use std::io::Write;

    // --8<-- [start:input-test]
    #[tokio::test(flavor = "current_thread")]
    async fn invalid_input_finishes_before_client_or_network_work() {
        for input in ["/notes", "file:///tmp/notes.txt"] {
            let settings = Settings {
                url: input.to_owned(),
                body: false,
                direct: false,
            };
            let mut output = Vec::new();
            let result = run(&settings, &mut output).await;
            match result {
                Err(AppError::Url(_)) => assert_eq!(input, "/notes"),
                Err(AppError::Scheme(scheme)) => assert_eq!(scheme.as_str(), "file"),
                other => panic!("兩種固定輸入應先拒絕：{other:?}"),
            }
            assert_eq!(output.len(), 0);
        }
    }
    // --8<-- [end:input-test]

    // --8<-- [start:render-test]
    #[test]
    fn a_given_http_report_keeps_status_time_and_body_choice() {
        let mut inspection = Inspection {
            final_url: "https://example.org/missing".to_owned(),
            status: 404,
            passed: false,
            body: None,
            elapsed_ms: 12,
        };
        let mut output = Vec::new();
        render(&inspection, &mut output).expect("Vec 接收固定報告");
        assert_eq!(
            output.as_slice(),
            b"final URL: https://example.org/missing\nHTTP status: 404\npassed: false\nelapsed: 12 ms\n"
        );
        inspection.body = Some(String::new());
        let mut output = Vec::new();
        render(&inspection, &mut output).expect("Vec 接收固定空 body");
        assert_eq!(
            output.as_slice(),
            b"final URL: https://example.org/missing\nHTTP status: 404\npassed: false\nelapsed: 12 ms\nbody:\n\n"
        );
    }
    // --8<-- [end:render-test]

    // --8<-- [start:writer-test]
    struct RefuseFlush {
        bytes: Vec<u8>,
    }
    impl Write for RefuseFlush {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            for byte in buffer {
                self.bytes.push(*byte);
            }
            Ok(buffer.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::from(io::ErrorKind::PermissionDenied))
        }
    }

    #[test]
    fn receiving_report_bytes_does_not_hide_a_later_flush_failure() {
        let inspection = Inspection {
            final_url: "https://example.org/notes".to_owned(),
            status: 200,
            passed: true,
            body: None,
            elapsed_ms: 0,
        };
        let mut output = RefuseFlush { bytes: Vec::new() };
        match render(&inspection, &mut output) {
            Err(error) => assert_eq!(error.kind(), io::ErrorKind::PermissionDenied),
            Ok(()) => panic!("flush 故意失敗，不應回成功"),
        }
        assert!(!output.bytes.as_slice().is_empty());
    }
    // --8<-- [end:writer-test]
}
