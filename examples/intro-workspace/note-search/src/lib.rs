pub mod config;
pub mod error;
pub mod model;
pub mod render;
pub mod search;

// --8<-- [start:run]
use crate::config::Settings;
use crate::error::AppError;
use crate::model::Report;
use crate::render::render;
use crate::search::search;
use crate::search::validate_query;
use std::fs;
use std::io::Write;

pub fn read_report(settings: &Settings) -> Result<Report, AppError> {
    validate_query(settings.query.as_str())?;
    let contents = match fs::read_to_string(settings.filename.as_path()) {
        Ok(contents) => contents,
        Err(error) => return Err(AppError::Read(error)),
    };
    search(
        settings.query.as_str(),
        contents.as_str(),
        settings.ignore_case,
    )
}

pub fn write_report<W: Write>(report: &Report, json: bool, output: &mut W) -> Result<(), AppError> {
    let text = render(report, json)?;
    let view = text.as_str();
    match output.write_all(view.as_bytes()) {
        Ok(()) => Ok(()),
        Err(error) => Err(AppError::Output(error)),
    }
}

pub fn run<W: Write>(settings: &Settings, output: &mut W) -> Result<(), AppError> {
    let report = read_report(settings)?;
    write_report(&report, settings.json, output)
}
// --8<-- [end:run]
