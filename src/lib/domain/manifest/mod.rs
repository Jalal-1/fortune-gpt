//! Structures, errors, and implementation for the [`Manifest`](crate::Manifest) data type.
pub mod field;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The possible errors that can occur when building a [`Manifest`]
#[derive(Debug, Error)]
pub enum ManifestError {
    /// Invalid title.
    #[error("Empty title")]
    EmptyTitle,

    /// Invalid description.
    #[error("Empty description")]
    EmptyDescription,

    /// Invalid Manifest escrow ID.
    #[error("Empty escrow ID")]
    EmptyManifestEscrowId,

    /// Invalid description.
    #[error("invalid description: {0}")]
    InvalidDescription(String),

    /// Invalid fortunes required field.
    #[error("invalid fortunes_required: {0}")]
    InvalidResponseRequired(String),

    /// Invalid fund amount
    #[error("empty escrow_id")]
    InvalidFundAmount,

    /// Date is invalid: invalid day of the month, too far in the past, etc.
    #[error("invalid date: {0}")]
    InvalidDate(String),

    /// Date failed to parse.
    #[error("date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),

    /// [crate::data::DbId] failed to parse.
    #[error("id parse error: {0}")]
    Id(#[from] uuid::Error),
    // Number of responses is negative or not a number.
    // #[error("responses parse error: {0}")]
    // Responses(#[from] std::num::TryFromIntError),
}

/// Manifest stores all the data about Manifests posted to the service.
///
/// Each field in the Manifest uses a newtype that encapsulates the requirements
/// for that particular field. If one of the fields cannot be created, then
/// a Manifest cannot be created. This enforcement of field creation ensures
/// that a Manifest will always be valid whenever it is utilized at any point
/// in the program.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Manifest {
    #[serde(skip)]
    /// The internal [`DbId`](crate::data::DbId) for the Manifest.
    pub manifest_id: field::ManifestId,
    pub manifest_escrow_id: field::ManifestEscrowId,
    pub title: field::Title,
    pub description: field::Description,
}
