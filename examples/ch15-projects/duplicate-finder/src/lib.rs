mod app;
mod collection;
mod content;
mod files;
mod metrics;
mod report;
mod stream;

pub use app::RunError;
pub use app::Settings;
pub use app::run;
pub use app::scan_report;
pub use app::write_output;
pub use app::write_report;
pub use collection::RootError;
pub use metrics::MetricError;
pub use report::Diagnostic;
pub use report::Report;
pub use report::ReportError;
pub use report::ReportGroup;
pub use report::render_json;
pub use report::render_text;

#[cfg(test)]
mod tests;
