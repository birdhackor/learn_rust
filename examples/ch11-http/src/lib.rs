// --8<-- [start:imports]
pub mod config;
pub mod error;

use config::Settings;
use error::AppError;
use reqwest::blocking::Client;
use reqwest::redirect::Policy;
use std::io;
use std::io::Write;
use std::time::Duration;
use url::Url;
// --8<-- [end:imports]

// --8<-- [start:model]
#[derive(Debug, PartialEq)]
pub struct Inspection {
    pub final_url: String,
    pub status: u16,
    pub passed: bool,
    pub body: Option<String>,
}
// --8<-- [end:model]

// --8<-- [start:parse-url]
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
// --8<-- [end:parse-url]

// --8<-- [start:inspect]
pub fn inspect(settings: &Settings) -> Result<Inspection, AppError> {
    let url = parse_http_url(settings.url.as_str())?;
    let mut builder = Client::builder()
        .redirect(Policy::limited(5))
        .timeout(Duration::from_secs(10));
    if settings.direct {
        builder = builder.no_proxy();
    }
    let client = match builder.build() {
        Ok(client) => client,
        Err(error) => return Err(AppError::Client(error)),
    };
    let response = match client.get(url).send() {
        Ok(response) => response,
        Err(error) => return Err(AppError::Request(error)),
    };
    let status = response.status();
    let final_url = response.url().as_str().to_owned();
    let passed = status.is_success();
    let body = if settings.body {
        match response.text() {
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
    })
}
// --8<-- [end:inspect]

// --8<-- [start:render-run]
pub fn render<W: Write>(inspection: &Inspection, output: &mut W) -> io::Result<()> {
    writeln!(output, "final URL: {}", inspection.final_url)?;
    writeln!(output, "HTTP status: {}", inspection.status)?;
    writeln!(output, "passed: {}", inspection.passed)?;
    if let Some(body) = &inspection.body {
        writeln!(output, "body:\n{body}")?;
    }
    output.flush()
}

pub fn run<W: Write>(settings: &Settings, output: &mut W) -> Result<bool, AppError> {
    let inspection = inspect(settings)?;
    match render(&inspection, output) {
        Ok(()) => Ok(inspection.passed),
        Err(error) => Err(AppError::Output(error)),
    }
}
// --8<-- [end:render-run]

// --8<-- [start:unit-tests]
#[cfg(test)]
mod tests {
    use crate::Inspection;
    use crate::error::AppError;
    use crate::parse_http_url;
    use crate::render;
    use std::io;
    use std::io::Write;

    #[test]
    fn scheme_check_uses_parsed_url_and_keeps_normalized_address() {
        let url = parse_http_url("HTTPS://example.org/notes?lang=rust#intro")
            .expect("合法 HTTPS URL 應可接受");
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.as_str(), "https://example.org/notes?lang=rust#intro");
    }

    #[test]
    fn malformed_and_unsupported_urls_are_different_input_errors() {
        match parse_http_url("/notes") {
            Err(AppError::Url(_)) => {}
            other => panic!("相對路徑應是解析錯誤：{other:?}"),
        }
        match parse_http_url("file:///tmp/notes.txt") {
            Err(AppError::Scheme(scheme)) => assert_eq!(scheme.as_str(), "file"),
            other => panic!("合法但不支援的 scheme 應另外分類：{other:?}"),
        }
    }

    #[test]
    fn http_not_passed_remains_a_report_and_body_is_an_explicit_choice() {
        let mut inspection = Inspection {
            final_url: "https://example.org/missing".to_owned(),
            status: 404,
            passed: false,
            body: None,
        };
        let mut output = Vec::new();
        render(&inspection, &mut output).expect("Vec 能接收輸出");
        assert_eq!(
            output.as_slice(),
            b"final URL: https://example.org/missing\nHTTP status: 404\npassed: false\n"
        );
        inspection.body = Some("missing".to_owned());
        let mut output = Vec::new();
        render(&inspection, &mut output).expect("Vec 能接收輸出");
        assert_eq!(
            output.as_slice(),
            b"final URL: https://example.org/missing\nHTTP status: 404\npassed: false\nbody:\nmissing\n"
        );
    }

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
    fn accepted_report_bytes_do_not_hide_a_flush_failure() {
        let inspection = Inspection {
            final_url: "https://example.org/notes".to_owned(),
            status: 200,
            passed: true,
            body: None,
        };
        let mut output = RefuseFlush { bytes: Vec::new() };
        match render(&inspection, &mut output) {
            Err(error) => assert_eq!(error.kind(), io::ErrorKind::PermissionDenied),
            Ok(()) => panic!("flush 已拒絕，輸出不能算完成"),
        }
        assert!(!output.bytes.as_slice().is_empty());
    }
}
// --8<-- [end:unit-tests]
