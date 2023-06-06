use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};

use crate::domain::manifest::ManifestError;

/// The title field for a [`Manifest`](crate::domain::manifest::Manifest).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ManifestEscrowId(String);

impl ManifestEscrowId {
    /// Create a new `ManifestEscrowId` field.
    ///
    /// If the title provided is empty, then a [`ManifestError`] will be returned.
    pub fn new(manifest_escrow_id: &str) -> Result<Self, ManifestError> {
        if !manifest_escrow_id.trim().is_empty() {
            Ok(Self(manifest_escrow_id.to_owned()))
        } else {
            Err(ManifestError::EmptyManifestEscrowId)
        }
    }
    /// Return the underlying [`String`].
    pub fn into_inner(self) -> String {
        self.0
    }
    /// Return a reference to the underlying [`&str`].
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for ManifestEscrowId {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self::new(field.value).map_err(|e| form::Error::validation(format!("{}", e)))?)
    }
}

#[cfg(test)]
mod test {
    use super::ManifestEscrowId;

    #[test]
    fn disallow_empty_title() {
        assert!(ManifestEscrowId::new("").is_err());
    }
}
