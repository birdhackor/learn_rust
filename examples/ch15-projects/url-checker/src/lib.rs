mod app;
mod batch;
mod config;
mod error;
mod input;
mod report;
mod single;

pub use app::check_text;
pub use app::run;
pub use app::write_output;
pub use app::write_report;
pub use batch::StopEvent;
pub use batch::default_jobs;
pub use config::Settings;
pub use error::RunError;
pub use report::Report;
pub use report::Row;
pub use report::render;
pub use single::build_client;
pub use single::default_limit;

#[cfg(test)]
mod test_fixture;
