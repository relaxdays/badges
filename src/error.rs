//! Error implementations

use thiserror::Error;

/// The default error type for this crate
#[derive(Debug, Error)]
pub enum BadgeError {
    /// Error wrapper around the [`askama::Error`] type from rendering
    #[error("Failed to render SVG: {0}")]
    RenderError(askama::Error),
    /// Error for trying to parse an invalid color
    #[error("Invalid color value: {0}")]
    InvalidColor(String),
}

impl From<askama::Error> for BadgeError {
    fn from(e: askama::Error) -> Self {
        Self::RenderError(e)
    }
}
