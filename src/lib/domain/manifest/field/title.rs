use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};

use crate::domain::manifest::ManifestError;

/// The title field for a [`Manifest`](crate::domain::manifest::Manifest).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Title(String);

impl Title {
    /// Create a new `Title` field.
    ///
    /// If the title provided is empty, then a [`ManifestError`] will be returned.
    pub fn new(title: &str) -> Result<Self, ManifestError> {
        if !title.trim().is_empty() {
            Ok(Self(title.to_owned()))
        } else {
            Err(ManifestError::EmptyTitle)
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
impl<'r> FromFormField<'r> for Title {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self::new(field.value).map_err(|e| form::Error::validation(format!("{}", e)))?)
    }
}

#[cfg(test)]
mod test {
    use super::Title;

    #[test]
    fn disallow_empty_title() {
        assert!(Title::new("").is_err());
    }
}
