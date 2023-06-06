use crate::data::DbId;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

/// The internal database id field for a [`Manifest`](crate::domain::manifest::Manifest).
#[derive(Clone, Debug, Constructor, Deserialize, Serialize)]
pub struct ManifestId(DbId);

impl ManifestId {
    /// Return the underlying [`DbId`](crate::data::DbId).
    pub fn into_inner(self) -> DbId {
        self.0
    }
}

impl From<DbId> for ManifestId {
    fn from(id: DbId) -> Self {
        Self(id)
    }
}

/// The Default implementation for [`ManifestId`] is an empty ID.
impl Default for ManifestId {
    fn default() -> Self {
        Self(DbId::nil())
    }
}
