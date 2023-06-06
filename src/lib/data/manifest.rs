//! Manifest data structures and functions.
use serde::{Deserialize, Serialize};

use crate::data::DbId;
use crate::domain::manifest::ManifestError;

/// Manifest that is fetched from a url and stored/retrieved later from the database.
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Manifest {
    pub(in crate::data) manifest_id: String,
    pub(in crate::data) manifest_escrow_id: String,
    pub(in crate::data) title: String,
    pub(in crate::data) description: String, // pub(in crate::data) fortunesRequired: i32,
                                             // pub(in crate::data) fundAmount: i32,
                                             // pub(in crate::data) fiat: bool,
                                             // pub(in crate::data) recordingOracleAddress: String,
                                             // pub(in crate::data) recordingOracleUrl: String,
}

/// Convert from a database model Manifest into a domain Manifest.
impl TryFrom<Manifest> for crate::domain::Manifest {
    type Error = ManifestError;
    fn try_from(req: Manifest) -> Result<Self, Self::Error> {
        use crate::domain::manifest::field;
        use std::str::FromStr;
        Ok(Self {
            manifest_id: field::ManifestId::new(DbId::from_str(req.manifest_id.as_str())?),
            manifest_escrow_id: field::ManifestEscrowId::new(req.manifest_escrow_id.as_str())?,
            title: field::Title::new(req.title.as_str())?,
            description: field::Description::new(req.description.as_str())?,
        })
    }
}
#[allow(dead_code)]
pub struct NewManifest {
    pub(in crate::data) manifest_id: String,
    pub(in crate::data) manifest_escrow_id: String,
    pub(in crate::data) title: String,
    pub(in crate::data) description: String,
    // pub(in crate::data) fortunesRequired: i32,
    // pub(in crate::data) fundAmount: i32,
    // pub(in crate::data) fiat: bool,
    // pub(in crate::data) recordingOracleAddress: String,
    // pub(in crate::data) recordingOracleUrl: String,
}

impl From<crate::service::ask::NewManifest> for NewManifest {
    fn from(req: crate::service::ask::NewManifest) -> Self {
        Self {
            manifest_id: DbId::new().into(),
            manifest_escrow_id: req.manifest_escrow_id.into_inner(),
            title: req.title.into_inner(),
            description: req.description.into_inner(),
            // expires: req.expires.into_inner().map(|time| time.timestamp()),
            // password: req.password.into_inner(),
            // shortcode: ShortCode::default().into(),
            // posted: Utc::now().timestamp(),
        }
    }
}
