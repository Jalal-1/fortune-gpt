//! Data structures to make a service request.

use crate::domain::job;
use crate::domain::manifest;
use crate::ShortCode;

use crate::domain::job::field::Password;
use serde::{Deserialize, Serialize};

/// Data required to run the [`new_job`](crate::service::action::new_job()) action to add a new [`crate::domain::Job`].
#[derive(Debug, Deserialize, Serialize)]
pub struct NewJob {
    pub escrow_id: job::field::EscrowId,
    pub manifest_url: job::field::ManifestUrl,
    pub posted: job::field::Posted,
    pub expires: job::field::Expires,
    pub password: job::field::Password,
}

/// Data required to run the [`update_job`](crate::service::action::update_job()) action to update [`crate::domain::Job`] data.
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateJob {
    pub escrow_id: job::field::EscrowId,
    pub manifest_url: job::field::ManifestUrl,
    pub expires: job::field::Expires,
    pub password: job::field::Password,
    pub shortcode: job::field::ShortCode,
}

/// Data required to run the [`get_job`](crate::service::action::get_job()) action to get a [`crate::domain::Job`].
#[derive(Debug, Deserialize, Serialize)]
pub struct GetJob {
    pub shortcode: ShortCode,
    pub password: job::field::Password,
}

impl GetJob {
    /// Convert a [`&str`] into a [`GetJob`] action request.
    pub fn from_raw(shortcode: &str) -> Self {
        Self {
            shortcode: ShortCode::from(shortcode),
            password: job::field::Password::default(),
        }
    }
}

impl From<ShortCode> for GetJob {
    fn from(shortcode: ShortCode) -> Self {
        Self {
            shortcode,
            password: Password::default(),
        }
    }
}

impl From<&str> for GetJob {
    fn from(raw: &str) -> Self {
        Self::from_raw(raw)
    }
}

/// Data required to run the [`new_manifest`](crate::service::action::new_manifest()) action to add a new [`crate::domain::Manifest`].
#[derive(Debug, Deserialize, Serialize)]
pub struct NewManifest {
    pub manifest_id: manifest::field::ManifestId,
    pub manifest_escrow_id: manifest::field::ManifestEscrowId,
    pub title: manifest::field::Title,
    pub description: manifest::field::Description,
}

/// Data required to run the [`get_manifest`](crate::service::action::get_manifest()) action to get a [`crate::domain::Manifest`].
#[derive(Debug, Deserialize, Serialize)]
pub struct GetManifest {
    pub manifest_escrow_id: manifest::field::ManifestEscrowId,
}
